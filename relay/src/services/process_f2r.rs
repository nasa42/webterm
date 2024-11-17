use crate::models::relay_error::RelayError;
use crate::models::send_payload::SendPayload;
use webterm_shared::flatbuffer_helpers::read_frontend_to_relay_message;
use webterm_shared::generated::flatbuffers_schema::{
    FrontendToRelayMessageType, RelayToAgentMessageType,
};

pub async fn process_f2r(data: Vec<u8>, send: &mut SendPayload) -> Result<(), RelayError> {
    let (type_, message) = read_frontend_to_relay_message(&data)?;

    match type_ {
        FrontendToRelayMessageType::ToAgent => {
            send.prepare_for_agent(
                None,
                RelayToAgentMessageType::FromFrontend,
                message.to_vec(),
            );
        }
        _ => {
            // do nothing
        }
    }

    Ok(())
}
