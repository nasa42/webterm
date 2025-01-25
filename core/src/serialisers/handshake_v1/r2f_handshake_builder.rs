use crate::generated::flatbuffers_schema::handshake_v1::{
    R2fHandshakeDevice, R2fHandshakeDeviceArgs, R2fHandshakeError, R2fHandshakeErrorArgs,
    R2fHandshakeErrorType, R2fHandshakeRoot, R2fHandshakeRootArgs, R2fHandshakeRootPayload,
    R2fHandshakeSuccess, R2fHandshakeSuccessArgs, Version,
};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct R2fHandshakeRootBlob(pub Vec<u8>);

pub struct R2fHandshakeBuilder<'a> {
    builder: FlatBufferBuilder<'a>,
    root_payload_type: R2fHandshakeRootPayload,
    root_payload: Option<WIPOffset<flatbuffers::UnionWIPOffset>>,
}

impl R2fHandshakeBuilder<'_> {
    pub fn new() -> Self {
        let builder = FlatBufferBuilder::new();
        Self {
            builder,
            root_payload_type: R2fHandshakeRootPayload::NONE,
            root_payload: None,
        }
    }

    pub fn root_payload_error(
        mut self,
        error_type: R2fHandshakeErrorType,
        message: Option<&str>,
    ) -> Self {
        let error_message_offset = message.map(|m| self.builder.create_string(m));
        let error = R2fHandshakeError::create(
            &mut self.builder,
            &R2fHandshakeErrorArgs {
                error_type,
                error_message: error_message_offset,
            },
        );
        self.root_payload_type = R2fHandshakeRootPayload::Error;
        self.root_payload = Some(error.as_union_value());
        self
    }

    pub fn root_payload_success(
        mut self,
        auth_nonce: &str,
        devices: Vec<(String, SystemTime)>,
    ) -> Self {
        let auth_nonce_offset = self.builder.create_string(auth_nonce);
        let devices_offset: Vec<WIPOffset<R2fHandshakeDevice>> = devices
            .iter()
            .map(|(subname, time)| {
                let subname_offset = self.builder.create_string(subname);
                let timestamp = time
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                R2fHandshakeDevice::create(
                    &mut self.builder,
                    &R2fHandshakeDeviceArgs {
                        subname: Some(subname_offset),
                        last_online_timestamp: timestamp,
                    },
                )
            })
            .collect();
        let devices_vector = self.builder.create_vector(&devices_offset);
        let payload = R2fHandshakeSuccess::create(
            &mut self.builder,
            &R2fHandshakeSuccessArgs {
                relay_auth_nonce: Some(auth_nonce_offset),
                devices: Some(devices_vector),
            },
        );
        self.root_payload_type = R2fHandshakeRootPayload::Success;
        self.root_payload = Some(payload.as_union_value());
        self
    }

    pub fn to_flatbuffers(mut self, relay_version: Version) -> R2fHandshakeRootBlob {
        let r2f_root = R2fHandshakeRoot::create(
            &mut self.builder,
            &R2fHandshakeRootArgs {
                relay_version: Some(&relay_version),
                root_payload_type: self.root_payload_type,
                root_payload: self.root_payload,
            },
        );

        self.builder.finish(r2f_root, None);
        R2fHandshakeRootBlob(self.builder.finished_data().to_vec())
    }
}
