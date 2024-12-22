use crate::config::Config;
use crate::messaging::process_f2a::process_f2a;
use crate::models::agent_error::AgentError;
use crate::models::send_payload::SendPayload;
use webterm_core::flatbuffers_helpers::read_message;
use webterm_core::generated::flatbuffers_schema::talk_v1::{
    R2aFromFrontend, R2aRoot, R2aRootPayload,
};
use webterm_core::types::FrontendId;

pub async fn process_r2a(
    message: &[u8],
    send: SendPayload,
    config: &Config,
) -> Result<SendPayload, AgentError> {
    let message = read_message::<R2aRoot>(message)?;
    let type_ = message.root_payload_type();

    match type_ {
        R2aRootPayload::FromFrontend => {
            let payload = message.root_payload_as_from_frontend();

            if let Some(payload) = payload {
                process_from_frontend(payload, send, config).await
            } else {
                Err(AgentError::FBParseError(
                    "R2aRootPayload::FromFrontend".to_string(),
                ))
            }
        }

        R2aRootPayload::RelayShuttingDown => process_relay_shutting_down(send).await,

        _ => Err(AgentError::FBParseError(format!(
            "Received unknown R2aRootPayload: {:?}",
            type_
        ))),
    }
}

async fn process_from_frontend(
    message: R2aFromFrontend<'_>,
    send: SendPayload,
    config: &Config,
) -> Result<SendPayload, AgentError> {
    process_f2a(
        FrontendId(message.frontend_id()),
        message.payload().unwrap_or_default().bytes(),
        send,
        config,
    )
    .await
}

async fn process_relay_shutting_down(mut send: SendPayload) -> Result<SendPayload, AgentError> {
    send.prepare_for_relay_shutdown();
    Ok(send)
}
