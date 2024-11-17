use crate::models::socket_reader::{SocketReader, SocketSubscriber};
use crate::models::socket_writer::{SocketPublisher, SocketWriter};
use axum::extract::ws::WebSocket;
use futures::StreamExt;
use tokio::sync::Notify;

pub struct AgentConnection {
    agent_writer: SocketWriter,
    agent_reader: SocketReader,
    close_notifier: Notify,
}

impl AgentConnection {
    pub async fn new(socket: WebSocket) -> Self {
        let (agent_writer, agent_reader) = socket.split();
        let agent_reader = SocketReader::new(agent_reader);
        let agent_writer = SocketWriter::new(agent_writer);

        Self {
            agent_writer,
            agent_reader,
            close_notifier: Notify::new(),
        }
    }

    pub async fn wait_until_closed(&self) {
        self.close_notifier.notified().await;
    }

    pub async fn close(&self) {
        self.close_notifier.notify_waiters();
    }

    pub fn publisher(&self) -> SocketPublisher {
        self.agent_writer.publisher()
    }

    pub fn subscriber(&self) -> SocketSubscriber {
        self.agent_reader.subscriber()
    }
}
