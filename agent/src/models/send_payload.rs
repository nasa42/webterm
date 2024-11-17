use crate::models::agent_error::AgentError;
use crate::models::socket_writer::SocketPublisher;
use crate::models::terminal::Terminal;
use crate::models::terminal_registry::TerminalRegistry;
use std::sync::Arc;
use tracing::debug;
use webterm_shared::flatbuffer_helpers::{
    create_agent_to_frontend_message, create_agent_to_relay_message,
};
use webterm_shared::generated::flatbuffers_schema::{
    AgentToFrontendMessageType, AgentToRelayMessageType,
};
use webterm_shared::pty_output_formatter::format_pty_output;

pub struct SendPayload {
    pub(crate) frontend_id: u64,
    to_relay: Option<Vec<u8>>,
    to_pty: Option<Vec<u8>>,
    to_pty_resize: Option<(u16, u16)>, // (cols, rows)
}

impl SendPayload {
    pub fn new(frontend_id: u64) -> Self {
        SendPayload {
            frontend_id,
            to_relay: None,
            to_pty: None,
            to_pty_resize: None,
        }
    }

    pub async fn dispatch(&self, relay_pub: &SocketPublisher) -> Result<(), AgentError> {
        if let Some(data) = &self.to_relay {
            debug!("dispatching to relay");
            relay_pub.send(data.to_owned()).await?;
        }

        if let Some(data) = &self.to_pty {
            debug!("dispatching to pty {:?}", format_pty_output(data));
            TerminalRegistry::singleton()
                .await
                .write_to_pty(self.frontend_id, data)
                .await?;
        }

        if let Some((cols, rows)) = &self.to_pty_resize {
            debug!("resizing pty");
            if let Some(terminal) = TerminalRegistry::singleton()
                .await
                .get_terminal(self.frontend_id)
                .await
            {
                terminal.resize(*cols, *rows).await?;
            } else {
                debug!("terminal not found");
            }
        }

        Ok(())
    }

    pub fn prepare_for_frontend(
        &mut self,
        frontend_id: u64,
        type_: AgentToFrontendMessageType,
        data: Vec<u8>,
    ) {
        let payload = create_agent_to_frontend_message(type_, data);
        debug!("prepare_for_frontend: {:?}", format_pty_output(&payload));
        self.prepare_for_relay(
            Some(frontend_id),
            AgentToRelayMessageType::ToFrontend,
            payload,
        );
    }

    pub fn prepare_for_relay(
        &mut self,
        frontend_id: Option<u64>,
        type_: AgentToRelayMessageType,
        data: Vec<u8>,
    ) {
        let payload = create_agent_to_relay_message(type_, data, frontend_id);

        self.to_relay = Some(payload);
    }

    pub fn prepare_for_pty(&mut self, data: Vec<u8>) {
        self.to_pty = Some(data);
    }

    pub fn prepare_for_pty_resize(&mut self, (cols, rows): (u16, u16)) {
        self.to_pty_resize = Some((cols, rows));
    }
}
