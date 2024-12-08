use crate::models::agent_connection::AgentConnection;
use crate::models::agent_registry::AgentRegistry;
use crate::models::frontend_connection::FrontendConnection;
use crate::models::handshake_nonce_registry::HandshakeNonceRegistry;
use crate::models::relay_error::RelayError;
use crate::models::send_payload::SendPayload;
use crate::models::socket_reader::SocketSubscriber;
use crate::models::socket_writer::SocketPublisher;
use crate::services::process_a2r::process_a2r;
use crate::services::process_f2r::process_f2r;
use axum::extract::ws::WebSocket;
use futures::stream::{SplitSink, SplitStream};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use webterm_core::pty_output_formatter::format_pty_output;
use webterm_core::types::FrontendId;

pub struct Bridge {
    frontend_id: FrontendId,
    frontend_connection: FrontendConnection,
    agent_connection: Arc<AgentConnection>,
}

impl Bridge {
    pub async fn connect_and_run(
        socket: WebSocket,
        handshake_nonce: String,
    ) -> Result<(), RelayError> {
        let session = Self::from_websocket(socket, handshake_nonce).await?;
        session.run_loop().await
    }

    pub async fn from_websocket(
        socket: WebSocket,
        handshake_nonce: String,
    ) -> Result<Self, RelayError> {
        // define frontend_connection first, so if subsequent code fails, frontend_connection is
        // dropped and socket is closed
        let frontend_connection = FrontendConnection::new(socket).await;

        let agent_connection = HandshakeNonceRegistry::singleton_frontend()
            .await
            .consume_nonce(&handshake_nonce)
            .await?
            .clone();

        Ok(Self::new(frontend_connection, agent_connection))
    }

    pub fn new(
        frontend_connection: FrontendConnection,
        agent_connection: Arc<AgentConnection>,
    ) -> Self {
        Self {
            frontend_id: agent_connection.next_frontend_id(),
            frontend_connection,
            agent_connection,
        }
    }

    pub async fn run_loop(self) -> Result<(), RelayError> {
        let fc = self.frontend_connection;
        let ac = self.agent_connection.clone();

        let frontend_id = self.frontend_id;

        let mut frontend_sub = fc.subscriber();
        let mut agent_sub = ac.subscriber();

        let frontend_pub = fc.publisher();
        let agent_pub = ac.publisher();

        let f2r_task = tokio::spawn(Self::f2r_task(
            frontend_sub,
            frontend_pub,
            agent_pub,
            frontend_id,
        ));

        let frontend_pub = fc.publisher();
        let agent_pub = ac.publisher();

        let a2r_task = tokio::spawn(Self::a2r_task(
            agent_sub,
            frontend_pub,
            agent_pub,
            frontend_id,
        ));

        tokio::select! {
            f2r_result = f2r_task => {
                match f2r_result {
                    Ok(result) => {
                        if let Err(e) = result {
                            error!("f2r_task encountered an error: {:?}", e);
                        } else {
                            info!("f2r_task exited successfully");
                        }
                    }
                    Err(e) => error!("f2r_task panicked: {:?}", e),
                }
            },
            a2r_result = a2r_task => {
                match a2r_result {
                    Ok(result) => {
                        if let Err(e) = result {
                            error!("a2r_task encountered an error: {:?}", e);
                        } else {
                            info!("a2r_task exited successfully");
                        }
                    }
                    Err(e) => error!("a2r_task panicked: {:?}", e),
                }
            }
        }

        fc.close().await;

        Ok(())
    }

    async fn f2r_task(
        mut frontend_sub: SocketSubscriber,
        frontend_pub: SocketPublisher,
        agent_pub: SocketPublisher,
        frontend_id: FrontendId,
    ) -> Result<(), RelayError> {
        info!("Starting f2r_task");
        loop {
            let data = frontend_sub.recv().await??;

            if let Some(data) = data {
                let mut send = SendPayload::new();
                let send = process_f2r(data, send, frontend_id).await?;
                send.dispatch(&frontend_pub, &agent_pub).await?;
            } else {
                continue;
            }
        }
    }

    async fn a2r_task(
        mut agent_sub: SocketSubscriber,
        frontend_pub: SocketPublisher,
        agent_pub: SocketPublisher,
        frontend_id: FrontendId,
    ) -> Result<(), RelayError> {
        info!("Starting a2r_task");
        loop {
            let data = agent_sub.recv().await??;

            let mut send = SendPayload::new();

            if let Some(data) = data {
                debug!("received from agent: {:?}", format_pty_output(&data));
                // TODO: avoid processing same a2r message for each frontend. It should be processed only once.
                let send = process_a2r(data, send).await?;
                let send_frontend_id = send.to_frontend_id();
                if send_frontend_id == Some(frontend_id) {
                    send.dispatch(&frontend_pub, &agent_pub).await?;
                } else {
                    continue;
                }
            } else {
                debug!("no data received from relay");
                continue;
            }
        }
    }
}
