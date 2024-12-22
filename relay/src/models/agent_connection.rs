use crate::models::socket_reader::{SocketReader, SocketSubscriber};
use crate::models::socket_writer::{SocketPublisher, SocketWriter};
use axum::extract::ws::WebSocket;
use futures::StreamExt;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::Notify;
use webterm_core::types::FrontendId;

pub struct AgentConnection {
    pub server_id: String,
    agent_writer: SocketWriter,
    agent_reader: SocketReader,
    close_notifier: Notify,
    next_frontend_id: AtomicU64,
}

impl AgentConnection {
    pub async fn new(server_id: String, socket: WebSocket) -> Self {
        let (agent_writer, agent_reader) = socket.split();
        let agent_reader = SocketReader::new(agent_reader);
        let agent_writer = SocketWriter::new(agent_writer);

        Self {
            server_id,
            agent_writer,
            agent_reader,
            close_notifier: Notify::new(),
            next_frontend_id: AtomicU64::new(1),
        }
    }

    pub async fn wait_until_closed(&self) {
        self.close_notifier.notified().await;
    }

    pub fn publisher(&self) -> SocketPublisher {
        self.agent_writer.publisher()
    }

    pub fn subscriber(&self) -> SocketSubscriber {
        self.agent_reader.subscriber()
    }

    pub fn next_frontend_id(&self) -> FrontendId {
        FrontendId(self.next_frontend_id.fetch_add(1, Ordering::SeqCst))
    }
}
