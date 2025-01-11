use crate::generated::flatbuffers_schema::talk_v1::activity::{
    PtyOutput, PtyOutputRoot, PtyOutputRootArgs,
};
use crate::generated::flatbuffers_schema::talk_v1::{VectorTable, VectorTableArgs};
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub struct ActivityInputBlob(pub Vec<u8>);
pub struct ActivityOutputBlob(pub Vec<u8>);

pub struct TerminalOutputBuilder<'a> {
    builder: FlatBufferBuilder<'a>,
    payload_type: PtyOutput,
    payload: Option<WIPOffset<flatbuffers::UnionWIPOffset>>,
}

impl TerminalOutputBuilder<'_> {
    pub fn new() -> Self {
        let builder = FlatBufferBuilder::new();
        Self {
            builder,
            payload_type: PtyOutput::NONE,
            payload: None,
        }
    }

    pub fn build_output(mut self, raw_output: &[u8]) -> Self {
        let data_offset = self.builder.create_vector(raw_output);
        let vector_offset = VectorTable::create(
            &mut self.builder,
            &VectorTableArgs {
                data: Some(data_offset),
            },
        );

        self.payload_type = PtyOutput::Output;
        self.payload = Some(vector_offset.as_union_value());
        self
    }

    pub fn to_flatbuffers(mut self, output_id: u64) -> ActivityOutputBlob {
        let root = PtyOutputRoot::create(
            &mut self.builder,
            &PtyOutputRootArgs {
                output_id,
                payload_type: self.payload_type,
                payload: self.payload,
            },
        );

        self.builder.finish(root, None);
        ActivityOutputBlob(self.builder.finished_data().to_vec())
    }
}
