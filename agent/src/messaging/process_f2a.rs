use crate::models::activity_registry::ActivityRegistry;
use crate::models::agent_error::AgentError;
use crate::models::send_payload::SendPayload;
use tracing::debug;
use webterm_shared::generated::flatbuffers_schema::talk_v1::F2aMessageType;
use webterm_shared::pty_output_formatter::format_pty_output;
use webterm_shared::talk_v1_helpers::{read_f2a_message, read_resize_message};

pub async fn process_f2a(message: &[u8], send: &mut SendPayload) -> Result<(), AgentError> {
    let message = read_f2a_message(message)?;
    let type_ = message.type_();
    let data = message.data().unwrap_or_default().bytes();

    match type_ {
        F2aMessageType::ActivityInput => {
            debug!("Received data from frontend: {}##", format_pty_output(data));
            send.prepare_for_pty(data.to_vec());
        }

        F2aMessageType::TerminalResize => {
            send.prepare_for_pty_resize(read_resize_message(data)?);
        }

        F2aMessageType::ActivityCreateTerminal => {
            debug!("received spawn terminal message");
            ActivityRegistry::singleton()
                .await
                .register_new(send.session_id)
                .await?;
        }

        _ => {
            // do nothing
        }
    }

    Ok(())
}
