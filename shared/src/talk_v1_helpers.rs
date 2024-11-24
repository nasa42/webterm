use crate::generated::flatbuffers_schema::talk_v1::{
    A2fMessage, A2fMessageArgs, A2fMessageType, A2rMessage, A2rMessageArgs, A2rMessageType,
    F2aMessage, F2rMessage, R2aMessage, R2aMessageArgs, R2aMessageType, R2fMessage, R2fMessageArgs,
    R2fMessageType, ResizeData, ResizeDataArgs,
};
use crate::types::SessionId;
use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer};

pub fn create_a2f_message(type_: A2fMessageType, data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = A2fMessageArgs {
        type_,
        data: Some(builder.create_vector(data.as_ref())),
    };

    let message = A2fMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn create_a2r_message(
    type_: A2rMessageType,
    data: impl AsRef<[u8]>,
    session_id: SessionId,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = A2rMessageArgs {
        type_,
        data: Some(builder.create_vector(data.as_ref())),
        session_id,
    };

    let message = A2rMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn create_r2a_message(
    type_: R2aMessageType,
    data: impl AsRef<[u8]>,
    session_id: SessionId,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = R2aMessageArgs {
        type_,
        data: Some(builder.create_vector(data.as_ref())),
        session_id,
    };

    let message = R2aMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}
pub fn create_r2f_message(type_: R2fMessageType, data: impl AsRef<[u8]>) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = R2fMessageArgs {
        type_,
        data: Some(builder.create_vector(data.as_ref())),
    };

    let message = R2fMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn create_resize_message(cols: u16, rows: u16) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = ResizeDataArgs { cols, rows };

    let message = ResizeData::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn read_a2r_message(data: &[u8]) -> Result<A2rMessage, InvalidFlatbuffer> {
    flatbuffers::root::<A2rMessage>(data)
}
pub fn read_f2r_message(data: &[u8]) -> Result<F2rMessage, InvalidFlatbuffer> {
    flatbuffers::root::<F2rMessage>(data)
}

pub fn read_f2a_message(data: &[u8]) -> Result<F2aMessage, InvalidFlatbuffer> {
    flatbuffers::root::<F2aMessage>(data)
}

pub fn read_r2a_message(data: &[u8]) -> Result<R2aMessage, InvalidFlatbuffer> {
    flatbuffers::root::<R2aMessage>(data)
}

pub fn read_resize_message(data: &[u8]) -> Result<ResizeData, InvalidFlatbuffer> {
    flatbuffers::root::<ResizeData>(data)
}
