use crate::models::activity_registry::ActivityRegistry;
use crate::models::agent_error::AgentError;
use crate::models::socket_writer::SocketPublisher;
use crate::models::terminal::Terminal;
use std::sync::Arc;
use tracing::debug;
use webterm_shared::generated::flatbuffers_schema::talk_v1::{
    A2fMessageType, A2rMessageType, ResizeData,
};
use webterm_shared::pty_output_formatter::format_pty_output;
use webterm_shared::talk_v1_helpers::{create_a2f_message, create_a2r_message};
use webterm_shared::types::SessionId;

pub struct SendPayload {
    pub(crate) session_id: SessionId,
    to_relay: Option<Vec<u8>>,
    to_pty: Option<Vec<u8>>,
    to_pty_resize: Option<(u16, u16)>, // (cols, rows)
    is_relay_shutdown: bool,
}

impl SendPayload {
    pub fn new(session_id: SessionId) -> Self {
        SendPayload {
            session_id,
            to_relay: None,
            to_pty: None,
            to_pty_resize: None,
            is_relay_shutdown: false,
        }
    }

    pub async fn dispatch(&self, relay_pub: &SocketPublisher) -> Result<(), AgentError> {
        if let Some(data) = &self.to_relay {
            debug!("dispatching to relay");
            relay_pub.send(data.to_owned()).await?;
        }

        if let Some(data) = &self.to_pty {
            debug!("dispatching to pty {:?}", format_pty_output(data));
            ActivityRegistry::singleton()
                .await
                .get_terminal_for_session(self.session_id)
                .await
                .ok_or(AgentError::RuntimeError(format!(
                    "terminal not found for session {}",
                    self.session_id
                )))?
                .write(data)
                .await?;
        }

        if let Some((cols, rows)) = &self.to_pty_resize {
            debug!("resizing pty");
            if let Some(terminal) = ActivityRegistry::singleton()
                .await
                .get_terminal_for_session(self.session_id)
                .await
            {
                terminal.resize(*cols, *rows).await?;
            } else {
                debug!("terminal not found");
            }
        }

        Ok(())
    }

    pub fn prepare_for_frontend(&mut self, type_: A2fMessageType, data: Vec<u8>) {
        let payload = create_a2f_message(type_, data);
        debug!("prepare_for_frontend: {:?}", format_pty_output(&payload));
        self.prepare_for_relay(A2rMessageType::ToFrontend, payload);
    }

    pub fn prepare_for_relay(&mut self, type_: A2rMessageType, data: Vec<u8>) {
        let payload = create_a2r_message(type_, data, self.session_id);

        self.to_relay = Some(payload);
    }

    pub fn prepare_for_pty(&mut self, data: Vec<u8>) {
        self.to_pty = Some(data);
    }

    pub fn prepare_for_pty_resize(&mut self, data: ResizeData) {
        self.to_pty_resize = Some((data.cols(), data.rows()));
    }

    pub fn prepare_for_relay_shutdown(&mut self) {
        self.is_relay_shutdown = true;
    }

    pub fn is_relay_shutdown(&self) -> bool {
        self.is_relay_shutdown
    }
}
