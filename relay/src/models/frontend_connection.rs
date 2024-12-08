use crate::models::socket_reader::{SocketReader, SocketSubscriber};
use crate::models::socket_writer::{SocketPublisher, SocketWriter};
use axum::extract::ws::WebSocket;
use futures::StreamExt;

pub struct FrontendConnection {
    frontend_writer: SocketWriter,
    frontend_reader: SocketReader,
}

impl FrontendConnection {
    pub async fn new(socket: WebSocket) -> Self {
        let (frontend_writer, frontend_reader) = socket.split();
        let frontend_reader = SocketReader::new(frontend_reader);
        let frontend_writer = SocketWriter::new(frontend_writer);
        Self {
            frontend_writer,
            frontend_reader,
        }
    }

    pub fn publisher(&self) -> SocketPublisher {
        self.frontend_writer.publisher()
    }

    pub fn subscriber(&self) -> SocketSubscriber {
        self.frontend_reader.subscriber()
    }

    pub async fn close(&self) {
        let _ = self.frontend_writer.close().await;
    }
}
