use crate::models::agent_error::AgentError;
use crate::models::relay::Relay;
use crate::models::socket_reader::{SocketReader, SocketSubscriber};
use crate::models::socket_writer::{SocketPublisher, SocketWriter};
use futures::StreamExt;
use tokio_tungstenite::connect_async;
use tracing::error;

pub struct RelayConnection {
    relay_writer: SocketWriter,
    relay_reader: SocketReader,
}

impl RelayConnection {
    pub async fn new(relay: &Relay) -> Result<Self, AgentError> {
        let socket = connect_async(relay.websocket_url()).await;

        if let Err(e) = socket {
            error!("Failed to connect to relay: {:?}", relay.websocket_url());
            return Err(AgentError::from(e));
        }

        let (socket, _) = socket?;

        let (relay_writer, relay_reader) = socket.split();

        Ok(Self {
            relay_writer: SocketWriter::new(relay_writer),
            relay_reader: SocketReader::new(relay_reader),
        })
    }

    pub fn publisher(&self) -> SocketPublisher {
        self.relay_writer.publisher()
    }

    pub fn subscriber(&self) -> SocketSubscriber {
        self.relay_reader.subscriber()
    }
}
