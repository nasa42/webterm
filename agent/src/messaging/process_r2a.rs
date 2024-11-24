use crate::messaging::process_f2a::process_f2a;
use crate::models::agent_error::AgentError;
use crate::models::send_payload::SendPayload;
use webterm_shared::generated::flatbuffers_schema::talk_v1::R2aMessageType;
use webterm_shared::talk_v1_helpers::read_r2a_message;

pub async fn process_r2a(message: &[u8]) -> Result<SendPayload, AgentError> {
    let message = read_r2a_message(message)?;
    let type_ = message.type_();
    let data = message.data().unwrap_or_default().bytes();
    let mut send = SendPayload::new(message.session_id());

    match type_ {
        R2aMessageType::FromFrontend => {
            process_f2a(data, &mut send).await?;
        }

        R2aMessageType::RelayShuttingDown => {
            send.prepare_for_relay_shutdown();
        }

        _ => {
            // do nothing
        }
    }

    Ok(send)
}
