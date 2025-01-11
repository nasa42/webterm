use crate::generated::flatbuffers_schema::talk_v1::{
    EmptyTable, EmptyTableArgs, R2fError, R2fErrorArgs, R2fErrorType, R2fFromAgent,
    R2fFromAgentArgs, R2fRoot, R2fRootArgs, R2fRootPayload,
};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct R2fRootBlob(pub Vec<u8>);

pub struct R2fBuilder<'a> {
    builder: FlatBufferBuilder<'a>,
    root_payload_type: R2fRootPayload,
    root_payload: Option<WIPOffset<flatbuffers::UnionWIPOffset>>,
}

impl R2fBuilder<'_> {
    pub fn new() -> Self {
        let builder = FlatBufferBuilder::new();
        Self {
            builder,
            root_payload_type: R2fRootPayload::NONE,
            root_payload: None,
        }
    }

    pub fn root_payload_error(mut self, error_type: R2fErrorType) -> Self {
        let error = R2fError::create(
            &mut self.builder,
            &R2fErrorArgs {
                error_type,
                error_message: None,
            },
        );
        self.root_payload_type = R2fRootPayload::Error;
        self.root_payload = Some(error.as_union_value());
        self
    }

    pub fn root_payload_from_agent(mut self, payload: &[u8]) -> Self {
        let payload_offset = self.builder.create_vector(payload);
        let from_agent = R2fFromAgent::create(
            &mut self.builder,
            &R2fFromAgentArgs {
                payload: Some(payload_offset),
            },
        );
        self.root_payload_type = R2fRootPayload::FromAgent;
        self.root_payload = Some(from_agent.as_union_value());
        self
    }

    pub fn root_payload_relay_shutting_down(mut self) -> Self {
        let empty_table = EmptyTable::create(&mut self.builder, &EmptyTableArgs {});
        self.root_payload_type = R2fRootPayload::RelayShuttingDown;
        self.root_payload = Some(empty_table.as_union_value());
        self
    }

    pub fn to_flatbuffers(mut self, message_id: u64) -> R2fRootBlob {
        let r2f_root = R2fRoot::create(
            &mut self.builder,
            &R2fRootArgs {
                message_id,
                root_payload_type: self.root_payload_type,
                root_payload: self.root_payload,
            },
        );

        self.builder.finish(r2f_root, None);
        R2fRootBlob(self.builder.finished_data().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flatbuffers::root;

    #[test]
    fn test_new_builder_initializes_correctly() {
        let builder = R2fBuilder::new();
        assert_eq!(builder.root_payload_type, R2fRootPayload::NONE);
        assert!(builder.root_payload.is_none());
    }

    #[test]
    fn test_root_payload_error() {
        let builder = R2fBuilder::new().root_payload_error(R2fErrorType::ErrorActivityNotFound);

        let blob = builder.to_flatbuffers();

        let root = root::<R2fRoot>(&blob.0).unwrap();

        assert_eq!(root.root_payload_type(), R2fRootPayload::Error);

        let error = root.root_payload_as_error().unwrap();
        assert_eq!(error.error_type(), R2fErrorType::ErrorActivityNotFound);
    }

    #[test]
    fn test_root_payload_from_agent() {
        let test_payload = b"test agent payload";
        let builder = R2fBuilder::new().root_payload_from_agent(test_payload);

        let blob = builder.to_flatbuffers();

        let root = root::<R2fRoot>(&blob.0).unwrap();

        assert_eq!(root.root_payload_type(), R2fRootPayload::FromAgent);

        let from_agent = root.root_payload_as_from_agent().unwrap();
        assert_eq!(from_agent.payload().unwrap().bytes(), test_payload);
    }

    #[test]
    fn test_root_payload_relay_shutting_down() {
        let builder = R2fBuilder::new().root_payload_relay_shutting_down();

        let blob = builder.to_flatbuffers();

        let root = root::<R2fRoot>(&blob.0).unwrap();

        assert_eq!(root.root_payload_type(), R2fRootPayload::RelayShuttingDown);

        assert!(root.root_payload_as_relay_shutting_down().is_some());
    }

    #[test]
    fn test_chained_builder_methods() {
        let builder = R2fBuilder::new()
            .root_payload_error(R2fErrorType::ErrorActivityNotFound)
            .root_payload_from_agent(b"secondary payload");

        let blob = builder.to_flatbuffers();

        let root = root::<R2fRoot>(&blob.0).unwrap();

        assert_eq!(root.root_payload_type(), R2fRootPayload::FromAgent);

        let from_agent = root.root_payload_as_from_agent().unwrap();
        assert_eq!(from_agent.payload().unwrap().bytes(), b"secondary payload");
    }

    #[test]
    fn test_blob_create_multiple_times() {
        let blob1 = R2fBuilder::new()
            .root_payload_error(R2fErrorType::ErrorActivityNotFound)
            .to_flatbuffers();

        let blob2 = R2fBuilder::new()
            .root_payload_relay_shutting_down()
            .to_flatbuffers();

        assert_ne!(blob1.0, blob2.0);

        let root1 = root::<R2fRoot>(&blob1.0).unwrap();
        let root2 = root::<R2fRoot>(&blob2.0).unwrap();

        assert_eq!(root1.root_payload_type(), R2fRootPayload::Error);
        assert_eq!(root2.root_payload_type(), R2fRootPayload::RelayShuttingDown);
    }
}
