use crate::models::relay_error::RelayError;
use crate::models::send_payload::SendPayload;
use webterm_shared::generated::flatbuffers_schema::talk_v1::{F2rMessageType, R2aMessageType};
use webterm_shared::talk_v1_helpers::read_f2r_message;
use webterm_shared::types::SessionId;

pub async fn process_f2r(
    message: Vec<u8>,
    send: &mut SendPayload,
    session_id: SessionId,
) -> Result<(), RelayError> {
    let message = read_f2r_message(&message)?;
    let type_ = message.type_();
    let data = message.data().unwrap_or_default().bytes();

    match type_ {
        F2rMessageType::ToAgent => {
            send.prepare_for_agent(session_id, R2aMessageType::FromFrontend, data.to_vec());
        }
        _ => {
            // do nothing
        }
    }

    Ok(())
}
