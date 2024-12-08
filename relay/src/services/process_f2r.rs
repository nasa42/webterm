use crate::models::relay_error::RelayError;
use crate::models::send_payload::SendPayload;
use webterm_core::flatbuffers_helpers::read_message;
use webterm_core::generated::flatbuffers_schema::talk_v1::{F2rRoot, F2rRootPayload};
use webterm_core::serialisers::talk_v1::r2a_builder::R2aBuilder;
use webterm_core::types::FrontendId;

pub async fn process_f2r(
    data: Vec<u8>,
    mut send: SendPayload,
    frontend_id: FrontendId,
) -> Result<SendPayload, RelayError> {
    let root = read_message::<F2rRoot>(&data)?;

    match root.root_payload_type() {
        F2rRootPayload::ToAgent => {
            let message = root
                .root_payload_as_to_agent()
                .ok_or(RelayError::FBParseError(
                    "Expected ToAgent message".to_string(),
                ))?;

            let r2a = R2aBuilder::new();
            let payload = r2a
                .root_payload_from_frontend(
                    frontend_id,
                    message
                        .payload()
                        .ok_or(RelayError::FBParseError(
                            "Expected payload in ToAgent message".to_string(),
                        ))?
                        .bytes(),
                )
                .to_flatbuffers();
            send.prepare_for_agent(payload)
        }

        _ => {
            return Err(RelayError::FBParseError(format!(
                "Unexpected root payload type: {:?}",
                root.root_payload_type()
            )))
        }
    }

    Ok(send)
}
