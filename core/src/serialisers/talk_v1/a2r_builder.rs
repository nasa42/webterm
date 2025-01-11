use crate::generated::flatbuffers_schema::talk_v1::{
    A2rError, A2rErrorArgs, A2rErrorType, A2rRoot, A2rRootArgs, A2rRootPayload, A2rToFrontend,
    A2rToFrontendArgs,
};
use crate::serialisers::talk_v1::a2f_builder::A2fRootBlob;
use crate::types::FrontendId;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct A2rRootBlob(pub Vec<u8>);

pub struct A2rBuilder<'a> {
    builder: FlatBufferBuilder<'a>,
    root_payload_type: A2rRootPayload,
    root_payload: Option<WIPOffset<flatbuffers::UnionWIPOffset>>,
}

impl A2rBuilder<'_> {
    pub fn new() -> Self {
        let builder = FlatBufferBuilder::new();
        Self {
            builder,
            root_payload_type: A2rRootPayload::NONE,
            root_payload: None,
        }
    }

    pub fn root_payload_error(mut self, error_type: A2rErrorType) -> Self {
        let error = A2rError::create(
            &mut self.builder,
            &A2rErrorArgs {
                error_type,
                error_message: None,
            },
        );
        self.root_payload_type = A2rRootPayload::Error;
        self.root_payload = Some(error.as_union_value());
        self
    }

    pub fn root_payload_to_frontend(
        mut self,
        frontend_id: FrontendId,
        payload: A2fRootBlob,
    ) -> Self {
        let payload_offset = self.builder.create_vector(&payload.0);
        let to_frontend = A2rToFrontend::create(
            &mut self.builder,
            &A2rToFrontendArgs {
                frontend_id: frontend_id.0,
                payload: Some(payload_offset),
            },
        );
        self.root_payload_type = A2rRootPayload::ToFrontend;
        self.root_payload = Some(to_frontend.as_union_value());
        self
    }

    pub fn to_flatbuffers(mut self, message_id: u64) -> A2rRootBlob {
        let a2r_root = A2rRoot::create(
            &mut self.builder,
            &A2rRootArgs {
                message_id,
                root_payload_type: self.root_payload_type,
                root_payload: self.root_payload,
            },
        );

        self.builder.finish(a2r_root, None);
        A2rRootBlob(self.builder.finished_data().to_vec())
    }
}
