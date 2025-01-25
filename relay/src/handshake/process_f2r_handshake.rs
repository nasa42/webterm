use crate::models::agent_registry::AgentRegistry;
use crate::models::handshake_nonce_frontend_registry::HandshakeNonceFrontendRegistry;
use std::time::SystemTime;
use tracing::debug;
use webterm_core::generated::flatbuffers_schema::handshake_v1::{
    F2rHandshakeRoot, F2rHandshakeRootPayload, R2fHandshakeErrorType,
};
use webterm_core::serialisers::handshake_v1::r2f_handshake_builder::R2fHandshakeBuilder;

pub async fn process_f2r_handshake(message: F2rHandshakeRoot<'_>) -> R2fHandshakeBuilder {
    let builder = R2fHandshakeBuilder::new();

    match message.root_payload_type() {
        F2rHandshakeRootPayload::RequestConnection => {
            let payload = message.root_payload_as_request_connection().unwrap();
            let device_name = payload.device_name().unwrap_or_default();

            if device_name.is_empty() {
                return builder.root_payload_error(
                    R2fHandshakeErrorType::ErrorAgentNotFound,
                    Some("Agent not found for an empty device name"),
                );
            }

            debug!("Processing F2rHandshake for device_name: {}", device_name);
            let subnames = AgentRegistry::subnames(device_name).await;

            if subnames.is_empty() {
                return builder.root_payload_error(
                    R2fHandshakeErrorType::ErrorAgentNotFound,
                    Some(format!("Agent not found for device: {}", device_name).as_str()),
                );
            }

            let auth_nonce = HandshakeNonceFrontendRegistry::singleton()
                .await
                .create_nonce(device_name.to_string(), None)
                .await;

            match auth_nonce {
                Err(_) => builder.root_payload_error(
                    R2fHandshakeErrorType::ErrorUnspecified,
                    Some(
                        format!("Failed to create auth_nonce for device: {}", device_name).as_str(),
                    ),
                ),
                Ok(auth_nonce) => builder.root_payload_success(
                    &auth_nonce,
                    subnames
                        .iter()
                        .map(|subname| (subname.to_string(), SystemTime::now()))
                        .collect(),
                ),
            }
        }
        _ => builder.root_payload_error(
            R2fHandshakeErrorType::ErrorUnspecified,
            Some(
                format!(
                    "Unknown root_payload_type: {:?}",
                    message.root_payload_type()
                )
                .as_str(),
            ),
        ),
    }
}
