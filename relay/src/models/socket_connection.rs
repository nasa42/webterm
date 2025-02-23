use crate::models::socket_reader::{SocketReader, SocketSubscriber};
use crate::models::socket_writer::{SocketPublisher, SocketWriter};
use axum::extract::ws::WebSocket;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Notify;

pub struct SocketConnection {
    writer: SocketWriter,
    reader: SocketReader,
    close_notifier: Arc<Notify>,
}

impl SocketConnection {
    pub fn new(socket: WebSocket) -> Self {
        let (writer, reader) = socket.split();
        let notifier = Arc::new(Notify::new());
        Self {
            writer: SocketWriter::new(writer),
            reader: SocketReader::new(reader, notifier.clone()),
            close_notifier: notifier.clone(),
        }
    }

    pub fn close_notifier(&self) -> Arc<Notify> {
        self.close_notifier.clone()
    }

    pub fn writer(&self) -> &SocketWriter {
        &self.writer
    }

    pub fn publisher(&self) -> SocketPublisher {
        self.writer.publisher()
    }

    pub fn subscriber(&self) -> SocketSubscriber {
        self.reader.subscriber()
    }
}
