use crate::models::agent_error::AgentError;
use crate::models::send_payload::SendPayload;
use crate::models::terminal_registry::TerminalRegistry;
use tracing::debug;
use webterm_shared::flatbuffer_helpers::{read_frontend_to_agent_message, read_resize_message};
use webterm_shared::generated::flatbuffers_schema::FrontendToAgentMessageType;
use webterm_shared::pty_output_formatter::format_pty_output;

pub async fn process_f2a(data: &[u8], send: &mut SendPayload) -> Result<(), AgentError> {
    let (type_, data) = read_frontend_to_agent_message(data)?;

    match type_ {
        FrontendToAgentMessageType::Data => {
            debug!("Received data from frontend: {}##", format_pty_output(data));
            send.prepare_for_pty(data.to_vec());
        }

        FrontendToAgentMessageType::Resize => {
            send.prepare_for_pty_resize(read_resize_message(data)?);
        }

        FrontendToAgentMessageType::SpawnTerminal => {
            debug!("received spawn terminal message");
            TerminalRegistry::singleton()
                .await
                .register_new(send.frontend_id)
                .await?;
        }

        _ => {
            // do nothing
        }
    }

    Ok(())
}
