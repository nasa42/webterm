use crate::models::relay_error::RelayError;
use crate::models::send_payload::SendPayload;
use webterm_shared::generated::flatbuffers_schema::talk_v1::{A2rMessageType, R2fMessageType};
use webterm_shared::talk_v1_helpers::read_a2r_message;
use webterm_shared::types::SessionId;

pub async fn process_a2r(data: Vec<u8>, send: &mut SendPayload) -> Result<SessionId, RelayError> {
    let message = read_a2r_message(&data)?;

    match message.type_() {
        A2rMessageType::ToFrontend => {
            send.prepare_for_frontend(
                R2fMessageType::FromAgent,
                message.data().unwrap_or_default().bytes().to_vec(),
            );
        }
        _ => {
            // do nothing
        }
    }

    Ok(message.session_id())
}
