use crate::config::Config;
use crate::models::agent_error::AgentError;
use crate::models::frontend_registry::FrontendRegistry;
use crate::models::send_payload::SendPayload;
use crate::models::session_registry::SessionRegistry;
use webterm_core::flatbuffers_helpers::read_message;
use webterm_core::generated::flatbuffers_schema::talk_v1::{
    A2fErrorType, F2aEncryptedRoot, F2aMessage, F2aMessageFormat, F2aPlainMessage, F2aRoot,
};
use webterm_core::models::webterm_error::WebtermError;
use webterm_core::serialisers::talk_v1::a2f_builder::A2fBuilder;
use webterm_core::serialisers::talk_v1::terminal_output_builder::ActivityInputBlob;
use webterm_core::types::{ActivityId, Bits96, FrontendId, SessionId};

pub async fn process_f2a(
    frontend_id: FrontendId,
    message: &[u8],
    send: SendPayload,
    config: &Config,
) -> Result<SendPayload, AgentError> {
    let root = read_message::<F2aRoot>(message)?;

    match root.format() {
        F2aMessageFormat::Plain => process_plain(root, frontend_id, send, config).await,
        _ => process_encrypted(root, frontend_id, send).await,
    }
}

async fn process_plain(
    message: F2aRoot<'_>,
    frontend_id: FrontendId,
    mut send: SendPayload,
    config: &Config,
) -> Result<SendPayload, AgentError> {
    let a2f = A2fBuilder::new();

    match message.plain_message_type() {
        F2aPlainMessage::AuthRequestPreamble => {
            let frontend = FrontendRegistry::build_frontend(frontend_id).await?;
            let frontend = frontend.lock().await;
            let version_str = env!("CARGO_PKG_VERSION");
            let version = semver::Version::parse(version_str).unwrap();
            let payload = a2f
                .build_preamble(
                    version,
                    frontend.salt(),
                    frontend.pbkdf2_iterations(),
                    frontend.challenge_nonce()?,
                )
                .to_flatbuffers_plain();

            send.prepare_for_frontend(frontend.frontend_id(), payload)
        }

        F2aPlainMessage::AuthPresentVerification => {
            let message = message
                .plain_message_as_auth_present_verification()
                .unwrap();
            let frontend_arc = FrontendRegistry::find(frontend_id).await?;
            let mut frontend = frontend_arc.lock().await;

            let mut success = false;
            frontend.init_cryptographer(config.secret_key());

            let decrypted = frontend.cryptographer()?.decrypt(
                message
                    .challenge_aes256gcm_solution()
                    .ok_or(AgentError::FBParseError(
                    "Expected challenge aes256gcm solution for auth present verification, got None"
                        .to_string(),
                ))?.bytes(),
                &Bits96::from(message.challenge_iv().ok_or(AgentError::FBParseError(
                    "Expected challenge iv for auth present verification, got None".to_string(),
                ))?),
                false,
            );

            if let Ok(decrypted) = decrypted {
                if decrypted == frontend.challenge_nonce()?.0.to_vec() {
                    success = true;
                }
            } else {
                success = false;
            }

            let session_arc = if SessionId(message.resume_session_id()) == SessionId(0) {
                SessionRegistry::build_session().await?
            } else {
                SessionRegistry::find(SessionId(message.resume_session_id())).await?
            };

            frontend.register_session(session_arc.clone());

            let mut session = session_arc.lock().await;
            session.set_current_frontend(frontend_arc.clone());
            frontend.register_session(session_arc.clone());
            let payload = a2f
                .build_auth_result(success, session.session_id())
                .to_flatbuffers_plain();

            send.prepare_for_frontend(frontend.frontend_id(), payload)
        }

        _ => {
            return Err(AgentError::FBParseError(format!(
                "Unknown plain message type: {:?}",
                message.plain_message_type()
            )))
        }
    }

    Ok(send)
}

async fn process_encrypted(
    root: F2aRoot<'_>,
    frontend_id: FrontendId,
    mut send: SendPayload,
) -> Result<SendPayload, AgentError> {
    let compressed = root.format() == F2aMessageFormat::Aes256GcmDeflateRaw;

    let frontend = FrontendRegistry::find(frontend_id).await?;
    let frontend = frontend.lock().await;

    let encrypted_payload = root.encrypted_payload();

    let encrypted_payload = encrypted_payload.ok_or(AgentError::FBParseError(format!(
        "Expected a2f encrypted payload, got None for frontend: {:?}",
        frontend.frontend_id(),
    )))?;

    let iv = Bits96::from(root.iv().ok_or(AgentError::FBParseError(
        "Expected iv for encrypted message, got None".to_string(),
    ))?);

    let decrypted =
        match frontend
            .cryptographer()?
            .decrypt(encrypted_payload.bytes(), &iv, compressed)
        {
            Ok(decrypted) => decrypted,
            Err(e) => {
                return match e {
                    WebtermError::DecryptionError(_) => {
                        let a2f = A2fBuilder::new();
                        let error_payload = a2f
                            .build_error(A2fErrorType::ErrorDecryptionFailed)
                            .to_flatbuffers_encrypted(frontend.cryptographer()?)?;
                        send.prepare_for_frontend(frontend.frontend_id(), error_payload);
                        Ok(send)
                    }
                    _ => Err(e.into()),
                }
            }
        };

    let message = read_message::<F2aEncryptedRoot>(&decrypted)?;

    let a2f = A2fBuilder::new();
    let session = frontend.session().await?;
    let session = session.lock().await;

    match message.message_type() {
        F2aMessage::ActivityInput => {
            let message = message
                .message_as_activity_input()
                .ok_or(AgentError::FBParseError(format!(
                    "Expected activity input for frontend: {:?}, got None",
                    frontend.frontend_id()
                )))?;
            let activity_id = ActivityId(message.activity_id());
            let input = message.input().ok_or(AgentError::FBParseError(format!(
                "Expected input for activity: {:?}, got None",
                activity_id
            )))?;

            let input = ActivityInputBlob(input.bytes().to_vec());
            let activity = session.get_activity(&activity_id).await?;
            send.prepare_for_activity(activity, input)
        }

        F2aMessage::ActivityCreateTerminal => {
            let activity = session.create_terminal_activity().await?;

            let payload = a2f
                .build_activity_create_terminal(activity.activity_id())
                .to_flatbuffers_encrypted(frontend.cryptographer()?)?;
            send.prepare_for_frontend(frontend.frontend_id(), payload);
        }

        _ => {
            return Err(AgentError::FBParseError(format!(
                "Unknown encrypted message type: {:#?}",
                message.message_type()
            )))
        }
    }

    Ok(send)
}
