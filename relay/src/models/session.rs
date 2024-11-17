use crate::models::agent_registry::AgentRegistry;
use crate::models::frontend_connection::FrontendConnection;
use crate::models::handshake_nonce_registry::HandshakeNonceRegistry;
use crate::models::relay_error::RelayError;
use crate::models::send_payload::SendPayload;
use crate::models::socket_reader::SocketSubscriber;
use crate::models::socket_writer::SocketPublisher;
use crate::services::process_a2r::process_a2r;
use crate::services::process_f2r::process_f2r;
use futures::stream::{SplitSink, SplitStream};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};
use webterm_shared::pty_output_formatter::format_pty_output;

pub struct Session {}

impl Session {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run_loop(
        self,
        frontend_connection: FrontendConnection,
        auth_nonce: String,
    ) -> Result<(), RelayError> {
        let agent_connection = HandshakeNonceRegistry::singleton_frontend()
            .await
            .consume_nonce(&auth_nonce)
            .await?
            .clone();
        let frontend_id = frontend_connection.frontend_id;

        let mut frontend_sub = frontend_connection.subscriber();
        let mut agent_sub = agent_connection.subscriber();

        let frontend_pub = frontend_connection.publisher();
        let agent_pub = agent_connection.publisher();

        let f2r_task = tokio::spawn(Self::f2r_task(frontend_sub, frontend_pub, agent_pub));

        let frontend_pub = frontend_connection.publisher();
        let agent_pub = agent_connection.publisher();

        let a2r_task = tokio::spawn(Self::a2r_task(
            agent_sub,
            frontend_pub,
            agent_pub,
            frontend_id,
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
    ) -> Result<(), RelayError> {
        info!("Starting f2r_task");
        loop {
            let data = frontend_sub.recv().await??;

            if let Some(data) = data {
                let mut send = SendPayload::new();
                process_f2r(data, &mut send).await?;
                send.dispatch(&frontend_pub, &agent_pub).await?;
            } else {
                continue;
            }
        }

        debug!("f2r_task exited");
        Ok(())
    }

    async fn a2r_task(
        mut agent_sub: SocketSubscriber,
        frontend_pub: SocketPublisher,
        agent_pub: SocketPublisher,
        frontend_id: u64,
    ) -> Result<(), RelayError> {
        info!("Starting a2r_task");
        loop {
            let data = agent_sub.recv().await??;

            let mut send = SendPayload::new();

            if let Some(data) = data {
                debug!("received from agent: {:?}", format_pty_output(&data));
                process_a2r(data, frontend_id, &mut send).await?;
                send.dispatch(&frontend_pub, &agent_pub).await?;
            } else {
                debug!("no data received from relay");
                continue;
            }
        }

        debug!("a2r_task exited");
        Ok(())
    }
}
