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
use webterm_shared::pty_output_formatter::format_pty_output;
use webterm_shared::types::SessionId;

pub struct Session {
    session_id: SessionId,
    frontend_connection: FrontendConnection,
    agent_connection: Arc<AgentConnection>,
}

impl Session {
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
        let agent_connection = HandshakeNonceRegistry::singleton_frontend()
            .await
            .consume_nonce(&handshake_nonce)
            .await?
            .clone();
        let frontend_connection = FrontendConnection::new(socket).await;

        Ok(Self::new(frontend_connection, agent_connection))
    }

    pub fn new(
        frontend_connection: FrontendConnection,
        agent_connection: Arc<AgentConnection>,
    ) -> Self {
        Self {
            session_id: agent_connection.next_session_id(),
            frontend_connection,
            agent_connection,
        }
    }

    pub async fn run_loop(self) -> Result<(), RelayError> {
        let fc = self.frontend_connection;
        let ac = self.agent_connection.clone();

        let session_id = self.session_id;

        let mut frontend_sub = fc.subscriber();
        let mut agent_sub = ac.subscriber();

        let frontend_pub = fc.publisher();
        let agent_pub = ac.publisher();

        let f2r_task = tokio::spawn(Self::f2r_task(
            frontend_sub,
            frontend_pub,
            agent_pub,
            session_id,
        ));

        let frontend_pub = fc.publisher();
        let agent_pub = ac.publisher();

        let a2r_task = tokio::spawn(Self::a2r_task(
            agent_sub,
            frontend_pub,
            agent_pub,
            session_id,
        ));

        tokio::select! {
            _ = f2r_task => {
                info!("f2r_task exited");
            },
            _ = a2r_task => {
                info!("a2r_task exited");
            }
        }

        Ok(())
    }

    async fn f2r_task(
        mut frontend_sub: SocketSubscriber,
        frontend_pub: SocketPublisher,
        agent_pub: SocketPublisher,
        session_id: SessionId,
    ) -> Result<(), RelayError> {
        info!("Starting f2r_task");
        loop {
            let data = frontend_sub.recv().await??;

            if let Some(data) = data {
                let mut send = SendPayload::new();
                process_f2r(data, &mut send, session_id).await?;
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
        session_id: SessionId,
    ) -> Result<(), RelayError> {
        info!("Starting a2r_task");
        loop {
            let data = agent_sub.recv().await??;

            let mut send = SendPayload::new();

            if let Some(data) = data {
                debug!("received from agent: {:?}", format_pty_output(&data));
                // TODO: avoid processing same a2r message for each frontend. It should be processed only once.
                let a2r_session_id = process_a2r(data, &mut send).await?;
                if a2r_session_id == session_id {
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
