use crate::cryptography::cryptographer::Cryptographer;
use crate::generated::flatbuffers_schema::talk_v1::{
    A2fActivityCreateResponse, A2fActivityCreateResponseArgs, A2fActivityOutput,
    A2fActivityOutputArgs, A2fEncryptedRoot, A2fEncryptedRootArgs, A2fError, A2fErrorArgs,
    A2fErrorType, A2fMessage, A2fMessageFormat, A2fPlainAuthPreamble, A2fPlainAuthPreambleArgs,
    A2fPlainAuthResult, A2fPlainAuthResultArgs, A2fPlainMessage, A2fRoot, A2fRootArgs, Version,
};
use crate::models::webterm_error::WebtermError;
use crate::types::{ActivityId, Bits256, SessionId};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct A2fRootBlob(pub Vec<u8>);

pub trait BuilderState {}
pub struct Initial;
pub struct EncryptionReady {
    encrypted_payload: (A2fMessage, WIPOffset<flatbuffers::UnionWIPOffset>),
}
pub struct PlainReady {
    plain_payload: (A2fPlainMessage, WIPOffset<flatbuffers::UnionWIPOffset>),
}

pub struct A2fBuilder<'a, State: BuilderState> {
    builder: FlatBufferBuilder<'a>,
    root_payload_state: State,
}

impl BuilderState for Initial {}
impl BuilderState for EncryptionReady {}
impl BuilderState for PlainReady {}

impl<'a> A2fBuilder<'a, Initial> {
    pub fn new() -> Self {
        Self {
            builder: FlatBufferBuilder::new(),
            root_payload_state: Initial,
        }
    }

    pub fn build_preamble(
        mut self,
        agent_version: semver::Version,
        salt: Bits256,
        pbkdf2_iterations: u32,
        challenge_nonce: Bits256,
    ) -> A2fBuilder<'a, PlainReady> {
        let version = Version::new(
            agent_version.major as u8,
            agent_version.minor as u8,
            agent_version.patch as u8,
        );
        let nonce_offset = self.builder.create_vector(&challenge_nonce.0);
        let preamble = A2fPlainAuthPreamble::create(
            &mut self.builder,
            &A2fPlainAuthPreambleArgs {
                agent_version: Some(&version),
                salt: Some(&salt.into()),
                pbkdf2_iterations,
                challenge_nonce: Some(nonce_offset),
            },
        );

        A2fBuilder {
            builder: self.builder,
            root_payload_state: PlainReady {
                plain_payload: (A2fPlainMessage::AuthPreamble, preamble.as_union_value()),
            },
        }
    }

    pub fn build_auth_result(
        mut self,
        success_auth: bool,
        session_id: SessionId,
    ) -> A2fBuilder<'a, PlainReady> {
        let response = A2fPlainAuthResult::create(
            &mut self.builder,
            &A2fPlainAuthResultArgs {
                success_auth,
                session_id: session_id.0,
            },
        );

        A2fBuilder {
            builder: self.builder,
            root_payload_state: PlainReady {
                plain_payload: (A2fPlainMessage::AuthResult, response.as_union_value()),
            },
        }
    }

    pub fn build_error(mut self, error_type: A2fErrorType) -> A2fBuilder<'a, EncryptionReady> {
        let error = A2fError::create(
            &mut self.builder,
            &A2fErrorArgs {
                error_type,
                error_message: None,
            },
        );

        A2fBuilder {
            builder: self.builder,
            root_payload_state: EncryptionReady {
                encrypted_payload: (A2fMessage::Error, error.as_union_value()),
            },
        }
    }

    pub fn build_activity_output(
        mut self,
        activity_id: ActivityId,
        output: &[u8],
    ) -> A2fBuilder<'a, EncryptionReady> {
        let output_offset = self.builder.create_vector(output);
        let activity_output = A2fActivityOutput::create(
            &mut self.builder,
            &A2fActivityOutputArgs {
                activity_id: activity_id.0,
                output: Some(output_offset),
            },
        );

        A2fBuilder {
            builder: self.builder,
            root_payload_state: EncryptionReady {
                encrypted_payload: (A2fMessage::ActivityOutput, activity_output.as_union_value()),
            },
        }
    }

    pub fn build_activity_create_terminal(
        mut self,
        activity_id: ActivityId,
    ) -> A2fBuilder<'a, EncryptionReady> {
        let response = A2fActivityCreateResponse::create(
            &mut self.builder,
            &A2fActivityCreateResponseArgs {
                success: true,
                activity_id: activity_id.0,
            },
        );

        A2fBuilder {
            builder: self.builder,
            root_payload_state: EncryptionReady {
                encrypted_payload: (
                    A2fMessage::ActivityCreateResponse,
                    response.as_union_value(),
                ),
            },
        }
    }
}

impl A2fBuilder<'_, EncryptionReady> {
    pub fn to_flatbuffers_encrypted(
        mut self,
        encryptor: &Cryptographer,
    ) -> Result<A2fRootBlob, WebtermError> {
        let (message_type, encrypted_message_offset) = self.root_payload_state.encrypted_payload;

        let encrypted_root = A2fEncryptedRoot::create(
            &mut self.builder,
            &A2fEncryptedRootArgs {
                message_type,
                message: Some(encrypted_message_offset),
            },
        );

        self.builder.finish(encrypted_root, None);
        let message_buffer = self.builder.finished_data();
        let response = encryptor.encrypt(message_buffer, true)?;

        let mut builder = FlatBufferBuilder::new();
        let encrypted_payload_offset = builder.create_vector(&response.ciphertext);

        let format = if response.compressed {
            A2fMessageFormat::Aes256GcmDeflateRaw
        } else {
            A2fMessageFormat::Aes256GcmUncompressed
        };

        let root = A2fRoot::create(
            &mut builder,
            &A2fRootArgs {
                format,
                iv: Some(&response.iv.into()),
                plain_message_type: A2fPlainMessage::NONE,
                plain_message: None,
                encrypted_payload: Some(encrypted_payload_offset),
            },
        );

        builder.finish(root, None);

        Ok(A2fRootBlob(builder.finished_data().to_vec()))
    }
}

impl A2fBuilder<'_, PlainReady> {
    pub fn to_flatbuffers_plain(mut self) -> A2fRootBlob {
        let (payload_type, payload) = self.root_payload_state.plain_payload;

        let root = A2fRoot::create(
            &mut self.builder,
            &A2fRootArgs {
                format: A2fMessageFormat::Plain,
                iv: None,
                plain_message_type: payload_type,
                plain_message: Some(payload.as_union_value()),
                encrypted_payload: None,
            },
        );

        self.builder.finish(root, None);
        A2fRootBlob(self.builder.finished_data().to_vec())
    }
}
