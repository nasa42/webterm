use crate::config::{
    DEVICE_NAME_MAX_LENGTH, DEVICE_NAME_MIN_LENGTH, DEVICE_SUBNAME_MAX_LENGTH,
    DEVICE_SUBNAME_MIN_LENGTH,
};
use crate::models::agent_registry::AgentRegistry;
use crate::models::handshake_nonce_agent_registry::HandshakeNonceAgentRegistry;
use webterm_core::generated::flatbuffers_schema::handshake_v1::{
    A2rHandshakeRoot, A2rHandshakeRootPayload, R2aHandshakeErrorType,
};
use webterm_core::models::device_id::DeviceId;
use webterm_core::serialisers::handshake_v1::r2a_handshake_builder::R2aHandshakeBuilder;

pub async fn process_a2r_handshake(message: A2rHandshakeRoot<'_>) -> R2aHandshakeBuilder {
    let builder = R2aHandshakeBuilder::new();

    match message.root_payload_type() {
        A2rHandshakeRootPayload::RequestConnection => {
            let payload = message.root_payload_as_request_connection().unwrap();
            let device_name = payload.device_name().unwrap_or_default();
            let device_subname = payload.device_subname().unwrap_or_default();

            let (valid, builder) =
                validate_device_name_and_subname(builder, device_name, device_subname);
            if !valid {
                return builder;
            }

            let device_id = DeviceId::new(device_name.to_string(), device_subname.to_string());

            if AgentRegistry::exists(&device_id).await {
                return builder.root_payload_error(
                    R2aHandshakeErrorType::ErrorDeviceAlreadyExists,
                    Some(
                        format!(
                            "Device with name: {} and subname: {} already exists",
                            device_name, device_subname
                        )
                        .as_str(),
                    ),
                );
            }

            let auth_nonce = HandshakeNonceAgentRegistry::singleton()
                .await
                .create_nonce(device_id)
                .await;

            match auth_nonce {
                Err(_) => builder.root_payload_error(
                    R2aHandshakeErrorType::ErrorUnspecified,
                    Some(
                        format!("Failed to create auth_nonce for device: {}", device_name).as_str(),
                    ),
                ),
                Ok(auth_nonce) => builder.root_payload_success(&auth_nonce),
            }
        }
        _ => builder.root_payload_error(
            R2aHandshakeErrorType::ErrorUnspecified,
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

fn validate_device_name_and_subname<'a>(
    builder: R2aHandshakeBuilder<'a>,
    device_name: &str,
    device_subname: &str,
) -> (bool, R2aHandshakeBuilder<'a>) {
    if device_name.len() < DEVICE_NAME_MIN_LENGTH {
        return (
            false,
            builder.root_payload_error(
                R2aHandshakeErrorType::ErrorDeviceNameTooShort,
                Some("Device name is too short"),
            ),
        );
    }

    if device_name.len() > DEVICE_NAME_MAX_LENGTH {
        return (
            false,
            builder.root_payload_error(
                R2aHandshakeErrorType::ErrorDeviceNameTooLong,
                Some("Device name is too long"),
            ),
        );
    }

    if device_name
        .chars()
        .any(|c| c == '/' || c.is_whitespace() || c.is_control())
    {
        return (
            false,
            builder.root_payload_error(
                R2aHandshakeErrorType::ErrorDeviceNameInvalidChars,
                Some("Device name can not contain '/', whitespaces, or non-printable characters"),
            ),
        );
    }

    if device_subname.len() < DEVICE_SUBNAME_MIN_LENGTH {
        return (
            false,
            builder.root_payload_error(
                R2aHandshakeErrorType::ErrorDeviceSubnameTooShort,
                Some("Device subname is too short"),
            ),
        );
    }

    if device_subname.len() > DEVICE_SUBNAME_MAX_LENGTH {
        return (
            false,
            builder.root_payload_error(
                R2aHandshakeErrorType::ErrorDeviceSubnameTooLong,
                Some("Device subname is too long"),
            ),
        );
    }

    if device_subname
        .chars()
        .any(|c| c == '/' || c.is_whitespace() || c.is_control())
    {
        return (
            false,
            builder.root_payload_error(
                R2aHandshakeErrorType::ErrorDeviceSubnameInvalidChars,
                Some(
                    "Device subname can not contain '/', whitespaces, or non-printable characters",
                ),
            ),
        );
    }

    (true, builder)
}
