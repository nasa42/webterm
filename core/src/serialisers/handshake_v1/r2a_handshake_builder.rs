use crate::generated::flatbuffers_schema::handshake_v1::{
    R2aHandshakeError, R2aHandshakeErrorArgs, R2aHandshakeErrorType, R2aHandshakeRoot,
    R2aHandshakeRootArgs, R2aHandshakeRootPayload, R2aHandshakeSuccess, R2aHandshakeSuccessArgs,
    Version,
};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct R2aHandshakeRootBlob(pub Vec<u8>);

pub struct R2aHandshakeBuilder<'a> {
    builder: FlatBufferBuilder<'a>,
    root_payload_type: R2aHandshakeRootPayload,
    root_payload: Option<WIPOffset<flatbuffers::UnionWIPOffset>>,
}

impl R2aHandshakeBuilder<'_> {
    pub fn new() -> Self {
        let builder = FlatBufferBuilder::new();
        Self {
            builder,
            root_payload_type: R2aHandshakeRootPayload::NONE,
            root_payload: None,
        }
    }

    pub fn root_payload_error(
        mut self,
        error_type: R2aHandshakeErrorType,
        message: Option<&str>,
    ) -> Self {
        let error_message_offset = message.map(|m| self.builder.create_string(m));
        let error = R2aHandshakeError::create(
            &mut self.builder,
            &R2aHandshakeErrorArgs {
                error_type,
                error_message: error_message_offset,
            },
        );
        self.root_payload_type = R2aHandshakeRootPayload::Error;
        self.root_payload = Some(error.as_union_value());
        self
    }

    pub fn root_payload_success(mut self, auth_nonce: &str) -> Self {
        let auth_nonce_offset = self.builder.create_string(auth_nonce);
        let payload = R2aHandshakeSuccess::create(
            &mut self.builder,
            &R2aHandshakeSuccessArgs {
                relay_auth_nonce: Some(auth_nonce_offset),
            },
        );
        self.root_payload_type = R2aHandshakeRootPayload::Success;
        self.root_payload = Some(payload.as_union_value());
        self
    }

    pub fn to_flatbuffers(mut self, relay_version: Version) -> R2aHandshakeRootBlob {
        let r2f_root = R2aHandshakeRoot::create(
            &mut self.builder,
            &R2aHandshakeRootArgs {
                relay_version: Some(&relay_version),
                root_payload_type: self.root_payload_type,
                root_payload: self.root_payload,
            },
        );

        self.builder.finish(r2f_root, None);
        R2aHandshakeRootBlob(self.builder.finished_data().to_vec())
    }
}
