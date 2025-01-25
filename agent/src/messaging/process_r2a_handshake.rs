use crate::models::agent_error::AgentError;
use tracing::error;
use webterm_core::generated::flatbuffers_schema::handshake_v1::{
    R2aHandshakeErrorType, R2aHandshakeRoot, R2aHandshakeRootPayload,
};

pub struct Output {
    pub auth_nonce: String,
}

pub async fn process_r2a_handshake(message: R2aHandshakeRoot<'_>) -> Result<Output, AgentError> {
    match message.root_payload_type() {
        R2aHandshakeRootPayload::Success => {
            let payload = message
                .root_payload_as_success()
                .ok_or(AgentError::RuntimeError(
                    "Failed to parse success root payload".to_string(),
                ))?;
            let nonce = payload.relay_auth_nonce();

            if let Some(nonce) = nonce {
                Ok(Output {
                    auth_nonce: nonce.to_string(),
                })
            } else {
                Err(AgentError::RuntimeError(
                    "Missing relay_auth_nonce".to_string(),
                ))
            }
        }
        R2aHandshakeRootPayload::Error => {
            let payload = message
                .root_payload_as_error()
                .ok_or(AgentError::RuntimeError(
                    "Failed to parse error root payload".to_string(),
                ))?;
            let error_message = payload.error_message();
            let error_type = payload.error_type();
            let message = format!("Error: {:?} ({:?})", error_message, error_type);
            error!(message);
            if error_type == R2aHandshakeErrorType::ErrorDeviceAlreadyExists {
                return Err(AgentError::RelayErrorAgentAlreadyExists);
            }
            Err(AgentError::RuntimeError(message.to_string()))
        }
        _ => Err(AgentError::RuntimeError(format!(
            "Unknown root_payload_type: {:?}",
            message.root_payload_type()
        ))),
    }
}
