use crate::config::{Config, RELAY_RECONNECT_INTERVAL};
use crate::models::activity_registry::ActivityRegistry;
use crate::models::agent_error::AgentError;
use crate::models::relay::Relay;
use crate::models::socket_reader::{SocketReader, SocketSubscriber};
use crate::models::socket_writer::{SocketPublisher, SocketWriter};
use futures::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Notify;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use tracing::{debug, error, info};

pub struct RelayConnection {
    config: Arc<Config>,
    state: RwLock<Option<State>>,
    connect_signal: Arc<Notify>,
    disconnect_signal: Arc<Notify>,
}

struct State {
    relay: Arc<Relay>,
    writer: SocketWriter,
    reader: SocketReader,
}

impl RelayConnection {
    pub async fn new(config: Arc<Config>) -> Arc<Self> {
        let record = Arc::new(Self {
            config,
            state: RwLock::new(None),
            connect_signal: Arc::new(Notify::new()),
            disconnect_signal: Arc::new(Notify::new()),
        });

        let record_clone = record.clone();
        tokio::spawn(async move {
            Self::connection_manager(record_clone).await;
        });

        record
    }

    async fn connection_manager(self: Arc<Self>) {
        loop {
            let relay = self.config.random_relay();
            match self.try_connect(relay.clone(), self.clone()).await {
                Ok(_) => {
                    info!("Connected to relay: {}", relay.websocket_url());
                    self.connect_signal.notify_waiters();
                    info!("Waiting for disconnect signal");
                    self.wait_for_disconnect().await;
                    info!("Received disconnect signal");
                }
                Err(e) => {
                    error!(
                        "Failed to connect to relay {}: {}",
                        relay.websocket_url(),
                        e
                    );
                }
            }

            sleep(RELAY_RECONNECT_INTERVAL).await;
        }
    }

    async fn connect(&self, relay: Arc<Relay>, self_arc: Arc<Self>) -> Result<(), AgentError> {
        debug!("Connecting to relay: {}", relay.websocket_url());
        let mut state = self.state.write().await;

        let socket = connect_async(relay.websocket_url()).await?;
        let (socket, _) = socket;
        let (relay_writer, relay_reader) = socket.split();

        *state = Some(State {
            relay,
            writer: SocketWriter::new(relay_writer, self_arc.clone()),
            reader: SocketReader::new(relay_reader, self_arc),
        });

        Ok(())
    }

    async fn try_connect(&self, relay: Arc<Relay>, self_arc: Arc<Self>) -> Result<(), AgentError> {
        if self.connected().await {
            return Ok(());
        }
        self.connect(relay, self_arc).await
    }

    pub async fn connected(&self) -> bool {
        self.state.read().await.is_some()
    }

    pub async fn publisher(&self) -> Option<SocketPublisher> {
        let state = self.state.read().await;
        state.as_ref().map(|s| s.writer.publisher())
    }

    pub async fn subscriber(&self) -> Option<SocketSubscriber> {
        let state = self.state.read().await;
        state.as_ref().map(|s| s.reader.subscriber())
    }

    pub async fn pub_sub(&self) -> Option<(SocketPublisher, SocketSubscriber)> {
        if !self.connected().await {
            return None;
        }

        let pub_ = self.publisher().await;
        let sub = self.subscriber().await;

        if let (Some(pub_), Some(sub)) = (pub_, sub) {
            Some((pub_, sub))
        } else {
            None
        }
    }

    // TODO: Is this race condition safe?
    // If the connect_signal.notify_waiters() is called by another task right after the connected()
    // check, and right before the notified() call, notified() will wait forever.
    //
    // One easy fix would be to have a separate task which calls connect_signal.notify_waiters()
    // every few seconds if connected() is true.
    pub async fn wait_for_connect(&self) {
        if self.connected().await {
            return;
        }
        self.connect_signal.notified().await;
    }

    pub async fn wait_for_disconnect(&self) {
        if !self.connected().await {
            return;
        }
        self.disconnect_signal.notified().await;
    }

    pub async fn disconnect(&self) {
        let mut state = self.state.write().await;
        *state = None;
        self.disconnect_signal.notify_waiters();
    }

    pub async fn disconnect_with_error(&self, e: AgentError) {
        error!("Disconnecting on error: {}", e);
        self.disconnect().await;
    }
}
