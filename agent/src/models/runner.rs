use crate::config::Config;
use crate::messaging::process_r2a::process_r2a;
use crate::models::activity_registry::ActivityRegistry;
use crate::models::agent_error::AgentError;
use crate::models::panic_error::PanicError;
use crate::models::pty_activity::PtyActivity;
use crate::models::pty_activity_reader::{PtyActivityReader, TerminalSubscriber};
use crate::models::relay_connection::RelayConnection;
use crate::models::send_payload::SendPayload;
use crate::models::session_registry::SessionRegistry;
use crate::models::socket_reader::SocketSubscriber;
use crate::models::socket_writer::SocketPublisher;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{debug, error, info};
use webterm_core::pty_output_formatter::format_pty_output;
use webterm_core::serialisers::talk_v1::a2f_builder::A2fBuilder;

pub struct Runner {}

impl Runner {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(self, config: Arc<Config>) -> Result<(), PanicError> {
        let rc = RelayConnection::new(config.clone()).await;

        loop {
            if let Some((relay_pub, relay_sub)) = rc.pub_sub().await {
                let r2a_task = tokio::spawn(Self::r2a_task(
                    relay_sub,
                    relay_pub.clone(),
                    rc.clone(),
                    config.clone(),
                ));

                let a2r_task = tokio::spawn(Self::a2r_task(relay_pub.clone(), rc.clone()));

                tokio::select! {
                    result = r2a_task => {
                        match result {
                            Ok(Ok(())) => {
                                info!("r2a_task exited with ok()");
                            }
                            Ok(Err(e)) => {
                                error!("r2a_task error: {}", e);
                            }
                            Err(e) => {
                                error!("r2a_task panic: {}", e);
                            }
                        }
                    },
                    result = a2r_task => {
                        match result {
                            Ok(Ok(())) => {
                                info!("a2r_task exited with ok()");
                            }
                            Ok(Err(e)) => {
                                error!("a2r_task error: {}", e);
                            }
                            Err(e) => {
                                error!("a2r_task panic: {}", e);
                            }
                        }
                    },
                    _ = rc.wait_for_disconnect() => {
                        info!("Received disconnect signal");
                    }
                }
            } else {
                rc.wait_for_connect().await;
            }
        }
    }

    async fn r2a_task(
        mut relay_sub: SocketSubscriber,
        relay_pub: SocketPublisher,
        rc: Arc<RelayConnection>,
        config: Arc<Config>,
    ) -> Result<(), AgentError> {
        loop {
            let data = relay_sub.recv().await;
            match data {
                Ok(Ok(Some(data))) => {
                    let send = SendPayload::new();
                    let send = process_r2a(&data, send, &config).await?;
                    if send.is_relay_shutdown() {
                        error!("Relay is shutting down");
                        rc.disconnect().await;
                        return Ok(());
                    }
                    send.dispatch(&relay_pub).await?
                }
                Err(e) => {
                    rc.disconnect_with_error(e.into()).await;
                    return Ok(());
                }
                _ => continue,
            }
        }
    }

    async fn a2r_task(
        relay_pub: SocketPublisher,
        rc: Arc<RelayConnection>,
    ) -> Result<(), AgentError> {
        let receiver = PtyActivityReader::receiver();

        loop {
            let output = receiver.lock().await.recv().await;
            if let Some(output) = output {
                let activity = ActivityRegistry::find(output.activity_id).await;
                if let Ok(activity) = activity {
                    let session = activity.parent_session().await;
                    if let Ok(session) = session {
                        let session = session.lock().await;
                        let frontend = session.current_frontend();
                        if let Ok(frontend) = frontend {
                            let frontend = frontend.lock().await;
                            let mut send = SendPayload::new();
                            let a2f = A2fBuilder::new();
                            let payload = a2f
                                .build_activity_output(output.activity_id, &output.to_fb_output().0)
                                .to_flatbuffers_encrypted(frontend.cryptographer()?)?;
                            send.prepare_for_frontend(frontend.frontend_id(), payload);
                            send.dispatch(&relay_pub).await?;
                        } else {
                            debug!(
                                "frontend not found for session_id: {:?}",
                                session.session_id()
                            );
                        }
                    } else {
                        debug!(
                            "session not found for activity_id: {:?}",
                            output.activity_id
                        )
                    }
                } else {
                    debug!(
                        "activity not found for activity_id: {:?}",
                        output.activity_id
                    );
                }
            }
        }
    }
}
