use crate::models::relay_error::RelayError;
use crate::models::socket_writer::SocketPublisher;
use tracing::debug;
use webterm_shared::flatbuffer_helpers::{
    create_relay_to_agent_message, create_relay_to_frontend_message,
};
use webterm_shared::generated::flatbuffers_schema::{
    RelayToAgentMessageType, RelayToFrontendMessageType,
};
use webterm_shared::pty_output_formatter::format_pty_output;

pub struct SendPayload {
    to_frontend: Option<Vec<u8>>,
    to_agent: Option<Vec<u8>>,
}

impl SendPayload {
    pub fn new() -> Self {
        SendPayload {
            to_frontend: None,
            to_agent: None,
        }
    }

    pub async fn dispatch(
        &self,
        frontend_pub: &SocketPublisher,
        agent_pub: &SocketPublisher,
    ) -> Result<(), RelayError> {
        if let Some(data) = &self.to_frontend {
            frontend_pub.send(data.to_owned()).await?;
        }

        if let Some(data) = &self.to_agent {
            agent_pub.send(data.to_owned()).await?;
        }

        Ok(())
    }

    pub fn prepare_for_frontend(&mut self, type_: RelayToFrontendMessageType, data: Vec<u8>) {
        debug!("prepare_for_frontend: {:?}", format_pty_output(&data));
        let payload = create_relay_to_frontend_message(type_, data);
        self.to_frontend = Some(payload);
    }

    pub fn prepare_for_agent(
        &mut self,
        frontend_id: Option<u64>,
        type_: RelayToAgentMessageType,
        data: Vec<u8>,
    ) {
        debug!("prepare_for_agent: {:?}", format_pty_output(&data));
        let payload = create_relay_to_agent_message(type_, data, frontend_id);

        self.to_agent = Some(payload);
    }
}
