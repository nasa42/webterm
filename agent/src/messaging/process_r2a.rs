use crate::messaging::process_f2a::process_f2a;
use crate::models::agent_error::AgentError;
use crate::models::send_payload::SendPayload;
use webterm_shared::flatbuffer_helpers::read_relay_to_agent_message;
use webterm_shared::generated::flatbuffers_schema::RelayToAgentMessageType;

pub async fn process_r2a(data: &[u8]) -> Result<SendPayload, AgentError> {
    let (type_, data, frontend_id) = read_relay_to_agent_message(data)?;
    let mut send = SendPayload::new(frontend_id);

    match type_ {
        RelayToAgentMessageType::FromFrontend => {
            process_f2a(data, &mut send).await?;
        }

        _ => {
            // do nothing
        }
    }

    Ok(send)
}
