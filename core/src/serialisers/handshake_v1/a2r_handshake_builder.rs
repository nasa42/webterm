use crate::generated::flatbuffers_schema::handshake_v1::{
    A2rHandshakeError, A2rHandshakeErrorArgs, A2rHandshakeErrorType, A2rHandshakeRequestConnection,
    A2rHandshakeRequestConnectionArgs, A2rHandshakeRoot, A2rHandshakeRootArgs,
    A2rHandshakeRootPayload, Version,
};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct A2rHandshakeRootBlob(pub Vec<u8>);

pub struct A2rHandshakeBuilder<'a> {
    builder: FlatBufferBuilder<'a>,
    root_payload_type: A2rHandshakeRootPayload,
    root_payload: Option<WIPOffset<flatbuffers::UnionWIPOffset>>,
}

impl A2rHandshakeBuilder<'_> {
    pub fn new() -> Self {
        let builder = FlatBufferBuilder::new();
        Self {
            builder,
            root_payload_type: A2rHandshakeRootPayload::NONE,
            root_payload: None,
        }
    }

    pub fn root_payload_error(
        mut self,
        error_type: A2rHandshakeErrorType,
        message: Option<&str>,
    ) -> Self {
        let error_message_offset = message.map(|m| self.builder.create_string(m));
        let error = A2rHandshakeError::create(
            &mut self.builder,
            &A2rHandshakeErrorArgs {
                error_type,
                error_message: error_message_offset,
            },
        );
        self.root_payload_type = A2rHandshakeRootPayload::Error;
        self.root_payload = Some(error.as_union_value());
        self
    }

    pub fn root_request_connection(mut self, device_name: &str, device_subname: &str) -> Self {
        let device_name_offset = self.builder.create_string(device_name);
        let device_subname_offset = self.builder.create_string(device_subname);

        let payload = A2rHandshakeRequestConnection::create(
            &mut self.builder,
            &A2rHandshakeRequestConnectionArgs {
                device_name: Some(device_name_offset),
                device_subname: Some(device_subname_offset),
            },
        );
        self.root_payload_type = A2rHandshakeRootPayload::RequestConnection;
        self.root_payload = Some(payload.as_union_value());
        self
    }

    pub fn to_flatbuffers(mut self, agent_version: Version) -> A2rHandshakeRootBlob {
        let r2f_root = A2rHandshakeRoot::create(
            &mut self.builder,
            &A2rHandshakeRootArgs {
                agent_version: Some(&agent_version),
                root_payload_type: self.root_payload_type,
                root_payload: self.root_payload,
            },
        );

        self.builder.finish(r2f_root, None);
        A2rHandshakeRootBlob(self.builder.finished_data().to_vec())
    }
}
