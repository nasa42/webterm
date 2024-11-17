use crate::generated::flatbuffers_schema::{
    AgentToFrontendMessage, AgentToFrontendMessageArgs, AgentToFrontendMessageType,
    AgentToRelayMessage, AgentToRelayMessageArgs, AgentToRelayMessageType, FrontendToAgentMessage,
    FrontendToAgentMessageType, FrontendToRelayMessage, FrontendToRelayMessageType,
    RelayToAgentMessage, RelayToAgentMessageArgs, RelayToAgentMessageType, RelayToFrontendMessage,
    RelayToFrontendMessageArgs, RelayToFrontendMessageType, ResizeData, ResizeDataArgs,
};
use flatbuffers::{FlatBufferBuilder, InvalidFlatbuffer};

pub fn create_agent_to_frontend_message(
    type_: AgentToFrontendMessageType,
    data: impl AsRef<[u8]>,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = AgentToFrontendMessageArgs {
        type_,
        data: Some(builder.create_vector(data.as_ref())),
    };

    let message = AgentToFrontendMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn create_agent_to_relay_message(
    type_: AgentToRelayMessageType,
    data: impl AsRef<[u8]>,
    frontend_id: Option<u64>,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = AgentToRelayMessageArgs {
        type_,
        data: Some(builder.create_vector(data.as_ref())),
        frontend_id: frontend_id.unwrap_or_default(),
    };

    let message = AgentToRelayMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn create_relay_to_agent_message(
    type_: RelayToAgentMessageType,
    data: impl AsRef<[u8]>,
    frontend_id: Option<u64>,
) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = RelayToAgentMessageArgs {
        type_,
        data: Some(builder.create_vector(data.as_ref())),
        frontend_id: frontend_id.unwrap_or_default(),
    };

    let message = RelayToAgentMessage::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

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

pub fn create_resize_message(cols: u16, rows: u16) -> Vec<u8> {
    let mut builder = FlatBufferBuilder::new();

    let args = ResizeDataArgs { cols, rows };

    let message = ResizeData::create(&mut builder, &args);

    builder.finish(message, None);
    builder.finished_data().to_vec()
}

pub fn read_agent_to_relay_message(
    data: &[u8],
) -> Result<(AgentToRelayMessageType, &[u8], u64), InvalidFlatbuffer> {
    let message = flatbuffers::root::<AgentToRelayMessage>(data)?;

    let type_ = message.type_();
    let data = message.data().unwrap_or_default().bytes();
    let frontend_id = message.frontend_id();

    Ok((type_, data, frontend_id))
}

pub fn read_frontend_to_relay_message(
    data: &[u8],
) -> Result<(FrontendToRelayMessageType, &[u8]), InvalidFlatbuffer> {
    let message = flatbuffers::root::<FrontendToRelayMessage>(data)?;

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

pub fn read_relay_to_agent_message(
    data: &[u8],
) -> Result<(RelayToAgentMessageType, &[u8], u64), InvalidFlatbuffer> {
    let message = flatbuffers::root::<RelayToAgentMessage>(data)?;

    let type_ = message.type_();
    let data = message.data().unwrap_or_default().bytes();
    let frontend_id = message.frontend_id();

    Ok((type_, data, frontend_id))
}

pub fn read_resize_message(data: &[u8]) -> Result<(u16, u16), InvalidFlatbuffer> {
    let message = flatbuffers::root::<ResizeData>(data)?;

    let cols = message.cols();
    let rows = message.rows();

    Ok((cols, rows))
}
