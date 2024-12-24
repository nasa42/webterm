use crate::models::activity::Activity;
use crate::models::agent_error::AgentError;
use crate::models::socket_writer::SocketPublisher;
use std::sync::Arc;
use tracing::debug;
use webterm_core::pty_output_formatter::format_pty_output;
use webterm_core::serialisers::talk_v1::a2f_builder::A2fRootBlob;
use webterm_core::serialisers::talk_v1::a2r_builder::{A2rBuilder, A2rRootBlob};
use webterm_core::serialisers::talk_v1::terminal_output_builder::ActivityInputBlob;
use webterm_core::types::FrontendId;

pub struct SendPayload {
    to_relay: Option<A2rRootBlob>,
    to_activity: Option<(Arc<Activity>, ActivityInputBlob)>,
    is_relay_shutdown: bool,
}

impl SendPayload {
    pub fn new() -> Self {
        SendPayload {
            to_relay: None,
            to_activity: None,
            is_relay_shutdown: false,
        }
    }

    pub async fn dispatch(self, relay_pub: &SocketPublisher) -> Result<(), AgentError> {
        if let Some(payload) = self.to_relay {
            // debug!("dispatching to relay");
            relay_pub.send(payload.0.into()).await?;
        }

        if let Some((activity, data)) = self.to_activity {
            debug!("dispatching to pty {:?}", format_pty_output(&data.0));
            activity.receive_input(data).await?;
        }

        Ok(())
    }

    pub fn prepare_for_frontend(&mut self, frontend_id: FrontendId, frontend_payload: A2fRootBlob) {
        // debug!(
        //     "prepare_for_frontend: {:?}",
        //     format_pty_output(&frontend_payload.0)
        // );
        let a2r = A2rBuilder::new();
        let payload = a2r
            .root_payload_to_frontend(frontend_id, frontend_payload)
            .to_flatbuffers();
        self.prepare_for_relay(payload);
    }

    pub fn prepare_for_relay(&mut self, data: A2rRootBlob) {
        self.to_relay = Some(data);
    }

    pub fn prepare_for_activity(&mut self, activity: Arc<Activity>, data: ActivityInputBlob) {
        self.to_activity = Some((activity, data));
    }

    pub fn prepare_for_relay_shutdown(&mut self) {
        self.is_relay_shutdown = true;
    }

    pub fn is_relay_shutdown(&self) -> bool {
        self.is_relay_shutdown
    }
}
