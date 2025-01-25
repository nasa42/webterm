use crate::models::agent_error::AgentError;
use crate::models::connection_manager::ConnectionManager;
use crate::models::relay::Relay;
use crate::models::socket_reader::{SocketReader, SocketSubscriber};
use crate::models::socket_writer::{SocketPublisher, SocketWriter};
use futures::StreamExt;
use std::sync::Arc;
use tokio_tungstenite::connect_async;
use tracing::debug;

pub struct RelayConnection {
    writer: SocketWriter,
    reader: SocketReader,
}

impl RelayConnection {
    pub async fn new(
        relay: Arc<Relay>,
        nonce: String,
        cm: Arc<ConnectionManager>,
    ) -> Result<Self, AgentError> {
        debug!("Connecting to relay: {}", relay.websocket_url(None));

        let socket = connect_async(relay.websocket_url(Some(nonce))).await?;
        let (socket, _) = socket;
        let (relay_writer, relay_reader) = socket.split();

        Ok(Self {
            writer: SocketWriter::new(relay_writer, cm.clone()),
            reader: SocketReader::new(relay_reader, cm),
        })
    }

    pub async fn publisher(&self) -> SocketPublisher {
        self.writer.publisher()
    }

    pub async fn subscriber(&self) -> SocketSubscriber {
        self.reader.subscriber()
    }
}
