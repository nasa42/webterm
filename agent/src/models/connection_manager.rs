use crate::config::{Config, RELAY_RECONNECT_INTERVAL};
use crate::models::agent_error::AgentError;
use crate::models::handshaker::Handshaker;
use crate::models::relay::Relay;
use crate::models::relay_connection::RelayConnection;
use crate::models::socket_reader::SocketSubscriber;
use crate::models::socket_writer::SocketPublisher;
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{error, info};

pub struct ConnectionManager {
    config: Arc<Config>,
    state: RwLock<Option<State>>,
    connect_signal: Arc<Notify>,
    disconnect_signal: Arc<Notify>,
}

struct State {
    rc: RelayConnection,
}

impl ConnectionManager {
    pub async fn new(config: Arc<Config>) -> Arc<Self> {
        let record = Arc::new(Self {
            config,
            state: RwLock::new(None),
            connect_signal: Arc::new(Notify::new()),
            disconnect_signal: Arc::new(Notify::new()),
        });

        let record_clone = record.clone();
        tokio::spawn(async move {
            Self::run(record_clone).await;
        });

        record
    }

    async fn run(self: Arc<Self>) {
        loop {
            let result = Handshaker::run(self.config.clone()).await;
            match result {
                Err(e) => {
                    error!("Failed to initiate handshake: {}", e)
                }
                Ok(handshake) => {
                    let relay = handshake.result_relay;
                    let nonce = handshake.result_nonce;
                    match self.try_connect(relay.clone(), self.clone(), nonce).await {
                        Ok(_) => {
                            info!("Connected to relay: {}", relay.websocket_url(None));
                            self.connect_signal.notify_waiters();
                            info!("Waiting for disconnect signal");
                            self.wait_for_disconnect().await;
                            info!("Received disconnect signal");
                        }
                        Err(e) => {
                            error!(
                                "Failed to connect to relay {}: {}",
                                relay.websocket_url(None),
                                e
                            );
                        }
                    }
                }
            }
            sleep(RELAY_RECONNECT_INTERVAL).await;
        }
    }

    async fn connect(
        &self,
        relay: Arc<Relay>,
        self_arc: Arc<Self>,
        nonce: String,
    ) -> Result<(), AgentError> {
        let rc = RelayConnection::new(relay.clone(), nonce, self_arc.clone()).await?;
        let mut state = self.state.write().await;
        *state = Some(State { rc });

        Ok(())
    }

    async fn try_connect(
        &self,
        relay: Arc<Relay>,
        self_arc: Arc<Self>,
        nonce: String,
    ) -> Result<(), AgentError> {
        if self.connected().await {
            return Ok(());
        }
        self.connect(relay, self_arc, nonce).await
    }

    pub async fn connected(&self) -> bool {
        self.state.read().await.is_some()
    }

    pub async fn pub_sub(&self) -> Option<(SocketPublisher, SocketSubscriber)> {
        if !self.connected().await {
            return None;
        }

        let state = self.state.read().await;

        if let Some(ref s) = *state {
            let pub_ = s.rc.publisher().await;
            let sub = s.rc.subscriber().await;

            return Some((pub_, sub));
        }

        None
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
