use crate::models::relay_error::RelayError;
use crate::models::socket_writer::SocketPublisher;
use tracing::debug;
use webterm_shared::generated::flatbuffers_schema::talk_v1::{R2aMessageType, R2fMessageType};
use webterm_shared::pty_output_formatter::format_pty_output;
use webterm_shared::talk_v1_helpers::{create_r2a_message, create_r2f_message};
use webterm_shared::types::SessionId;

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

    pub fn prepare_for_frontend(&mut self, type_: R2fMessageType, data: Vec<u8>) {
        debug!("prepare_for_frontend: {:?}", format_pty_output(&data));
        let payload = create_r2f_message(type_, data);
        self.to_frontend = Some(payload);
    }

    pub fn prepare_for_agent(
        &mut self,
        session_id: SessionId,
        type_: R2aMessageType,
        data: Vec<u8>,
    ) {
        debug!("prepare_for_agent: {:?}", format_pty_output(&data));
        let payload = create_r2a_message(type_, data, session_id);

        self.to_agent = Some(payload);
    }
}
