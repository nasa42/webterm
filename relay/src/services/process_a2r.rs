use crate::models::relay_error::RelayError;
use crate::models::send_payload::SendPayload;
use webterm_shared::flatbuffer_helpers::read_agent_to_relay_message;
use webterm_shared::generated::flatbuffers_schema::{
    AgentToRelayMessageType, RelayToFrontendMessageType,
};

pub async fn process_a2r(
    data: Vec<u8>,
    frontend_id: u64,
    send: &mut SendPayload,
) -> Result<(), RelayError> {
    let (type_, message, frontend_id_) = read_agent_to_relay_message(&data)?;

    // TODO: ensure message goes to the right frontend ID
    // if frontend_id != frontend_id_ {
    //     return Ok(());
    // }

    match type_ {
        AgentToRelayMessageType::ToFrontend => {
            send.prepare_for_frontend(RelayToFrontendMessageType::FromAgent, message.to_vec());
        }
        _ => {
            // do nothing
        }
    }

    Ok(())
}
