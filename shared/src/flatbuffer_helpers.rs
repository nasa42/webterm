use crate::generated::flatbuffers_schema::{
    root_as_frontend_to_relay_message, AgentToFrontendMessage, AgentToFrontendMessageArgs,
    AgentToFrontendMessageType, FrontendToAgentMessage, FrontendToAgentMessageType,
    FrontendToRelayMessageType, RelayToFrontendMessage, RelayToFrontendMessageArgs,
    RelayToFrontendMessageType, ResizeData, ResizeDataArgs,
};
use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer};

pub fn create_relay_to_frontend_message(
    type_: RelayToFrontendMessageType,
    data: impl AsRef<[u8]>,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = RelayToFrontendMessageArgs {
        type_,
        data: Some(builder.create_vector(data.as_ref())),
    };

    let message = RelayToFrontendMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn create_agent_to_frontend_message(type_: AgentToFrontendMessageType, data: &[u8]) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = AgentToFrontendMessageArgs {
        type_,
        data: Some(builder.create_vector(data)),
    };

    let message = AgentToFrontendMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn create_message(cols: u16, rows: u16) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = ResizeDataArgs { cols, rows };

    let message = ResizeData::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn read_frontend_to_relay_message(
    data: &[u8],
) -> Result<(FrontendToRelayMessageType, &[u8]), InvalidFlatbuffer> {
    let message = root_as_frontend_to_relay_message(data)?;

    let type_ = message.type_();
    let data = message.data().unwrap_or_default().bytes();

    Ok((type_, data))
}

pub fn read_frontend_to_agent_message(
    data: &[u8],
) -> Result<(FrontendToAgentMessageType, &[u8]), InvalidFlatbuffer> {
    let message = flatbuffers::root::<FrontendToAgentMessage>(data)?;

    let type_ = message.type_();
    let data = message.data().unwrap_or_default().bytes();

    Ok((type_, data))
}

pub fn read_message(data: &[u8]) -> Result<(u16, u16), InvalidFlatbuffer> {
    let message = flatbuffers::root::<ResizeData>(data)?;

    let cols = message.cols();
    let rows = message.rows();

    Ok((cols, rows))
}
