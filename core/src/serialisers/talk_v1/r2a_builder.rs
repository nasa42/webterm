use crate::generated::flatbuffers_schema::talk_v1::{
    EmptyTable, EmptyTableArgs, R2aError, R2aErrorArgs, R2aErrorType, R2aFromFrontend,
    R2aFromFrontendArgs, R2aRoot, R2aRootArgs, R2aRootPayload,
};
use crate::types::FrontendId;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct R2aRootBlob(pub Vec<u8>);

pub struct R2aBuilder<'a> {
    builder: FlatBufferBuilder<'a>,
    root_payload_type: R2aRootPayload,
    root_payload: Option<WIPOffset<flatbuffers::UnionWIPOffset>>,
}

impl R2aBuilder<'_> {
    pub fn new() -> Self {
        let builder = FlatBufferBuilder::new();
        Self {
            builder,
            root_payload_type: R2aRootPayload::NONE,
            root_payload: None,
        }
    }

    pub fn root_payload_error(mut self, error_type: R2aErrorType) -> Self {
        let error = R2aError::create(
            &mut self.builder,
            &R2aErrorArgs {
                error_type,
                error_message: None,
            },
        );
        self.root_payload_type = R2aRootPayload::Error;
        self.root_payload = Some(error.as_union_value());
        self
    }

    pub fn root_payload_from_frontend(mut self, frontend_id: FrontendId, payload: &[u8]) -> Self {
        let payload_offset = self.builder.create_vector(payload);
        let from_frontend = R2aFromFrontend::create(
            &mut self.builder,
            &R2aFromFrontendArgs {
                frontend_id: frontend_id.0,
                payload: Some(payload_offset),
            },
        );
        self.root_payload_type = R2aRootPayload::FromFrontend;
        self.root_payload = Some(from_frontend.as_union_value());
        self
    }

    pub fn root_payload_relay_shutting_down(mut self) -> Self {
        let empty_table = EmptyTable::create(&mut self.builder, &EmptyTableArgs {});
        self.root_payload_type = R2aRootPayload::RelayShuttingDown;
        self.root_payload = Some(empty_table.as_union_value());
        self
    }

    pub fn to_flatbuffers(mut self) -> R2aRootBlob {
        let r2a_root = R2aRoot::create(
            &mut self.builder,
            &R2aRootArgs {
                root_payload_type: self.root_payload_type,
                root_payload: self.root_payload,
            },
        );

        self.builder.finish(r2a_root, None);
        R2aRootBlob(self.builder.finished_data().to_vec())
    }
}
