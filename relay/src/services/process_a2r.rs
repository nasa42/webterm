use crate::models::relay_error::RelayError;
use crate::models::send_payload::SendPayload;
use tracing::debug;
use webterm_core::flatbuffers_helpers::read_message;
use webterm_core::generated::flatbuffers_schema::talk_v1::{A2rRoot, A2rRootPayload};
use webterm_core::serialisers::talk_v1::r2f_builder::R2fBuilder;
use webterm_core::types::FrontendId;

pub async fn process_a2r(data: Vec<u8>, mut send: SendPayload) -> Result<SendPayload, RelayError> {
    let root = read_message::<A2rRoot>(&data)?;

    debug!("message id from agent: {:?}", root.message_id());

    send.message_id = root.message_id();

    match root.root_payload_type() {
        A2rRootPayload::ToFrontend => {
            let message = root
                .root_payload_as_to_frontend()
                .ok_or(RelayError::FBParseError(
                    "Expected ToFrontend in A2rMessageType::ToFrontend".to_string(),
                ))?;

            let r2f = R2fBuilder::new();
            let payload = r2f
                .root_payload_from_agent(
                    message
                        .payload()
                        .ok_or(RelayError::FBParseError(
                            "Expected payload in A2rMessageType::ToFrontend".to_string(),
                        ))?
                        .bytes(),
                )
                .to_flatbuffers(root.message_id());

            send.prepare_for_frontend(FrontendId(message.frontend_id()), payload);
        }

        _ => {
            return Err(RelayError::FBParseError(format!(
                "Unknown A2rMessageType: {:?}",
                root.root_payload_type()
            )))
        }
    }

    Ok(send)
}
