use crate::config::Config;
use crate::messaging::process_r2a::process_r2a;
use crate::models::agent_error::AgentError;
use crate::models::relay_connection::RelayConnection;
use crate::models::send_payload::SendPayload;
use crate::models::socket_reader::SocketSubscriber;
use crate::models::socket_writer::SocketPublisher;
use crate::models::terminal::Terminal;
use crate::models::terminal_reader::{TerminalReader, TerminalSubscriber};
use crate::models::terminal_registry::TerminalRegistry;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};
use webterm_shared::generated::flatbuffers_schema::AgentToFrontendMessageType;
use webterm_shared::pty_output_formatter::format_pty_output;

pub struct Runner {}

impl Runner {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self, mut config: Config) -> Result<(), AgentError> {
        let frontend_id: u64 = 1;

        let relay_connection = RelayConnection::new(&config.relay()).await?;

        let mut relay_sub = relay_connection.subscriber();
        let relay_pub = relay_connection.publisher();
        let r2a_task = tokio::spawn(Self::r2a_task(relay_sub, relay_pub));

        let relay_pub = relay_connection.publisher();
        let a2r_task = tokio::spawn(Self::a2r_task(relay_pub));

        tokio::select! {
            _ = r2a_task => {
                info!("r2a_task exited");
            },
            _ = a2r_task => {
                info!("a2r_task exited");
            }
        }

        Ok(())
    }

    async fn r2a_task(
        mut relay_sub: SocketSubscriber,
        relay_pub: SocketPublisher,
    ) -> Result<(), AgentError> {
        debug!("r2a_task started");
        loop {
            let data = relay_sub.recv().await??;

            if let Some(data) = data {
                let send = process_r2a(&data).await?;
                send.dispatch(&relay_pub).await?;
            } else {
                continue;
            }
        }
        debug!("r2a_task exited");
    }

    async fn a2r_task(relay_pub: SocketPublisher) -> Result<(), AgentError> {
        debug!("a2r_task started");
        let receiver = TerminalReader::receiver();
        loop {
            debug!("REACHED HERE in a2r_task");
            if let Some(data) = receiver.lock().await.recv().await {
                debug!(
                    "received from terminal: {:?}",
                    format_pty_output(&*data.data)
                );
                let mut send = SendPayload::new(data.frontend_id);

                send.prepare_for_frontend(
                    data.frontend_id,
                    AgentToFrontendMessageType::Data,
                    data.data,
                );
                send.dispatch(&relay_pub).await?;
            } else {
                debug!("a2r_task: receiver died");
                return Err(AgentError::RuntimeError(
                    "a2r_task failed: it should be impossible for this mpsc channel to die"
                        .to_string(),
                ));
            }
        }
        debug!("a2r_task exited");
    }
}
