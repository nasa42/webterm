use crate::models::relay_error::RelayError;
use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use futures::SinkExt;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info};

pub type SocketPublisher = mpsc::Sender<Vec<u8>>;

pub struct SocketWriter {
    _tx: SocketPublisher,
    writer_stream: Arc<Mutex<SplitSink<WebSocket, Message>>>,
}

impl SocketWriter {
    pub fn new(mut writer_stream: SplitSink<WebSocket, Message>) -> Self {
        let writer_stream = Arc::new(Mutex::new(writer_stream));
        let (_tx, mut rx) = mpsc::channel::<Vec<u8>>(16);
        let writer_stream_clone = writer_stream.clone();

        tokio::spawn(async move {
            loop {
                let received = rx.recv().await;
                match received {
                    None => {
                        info!("mpsc rx closed");
                        break;
                    }
                    Some(message) => {
                        let result = writer_stream
                            .lock()
                            .await
                            .send(Message::Binary(message))
                            .await;
                        match result {
                            Ok(_) => {
                                // continue
                            }
                            Err(error) => {
                                info!("Error sending message to writer stream: {:?}", error);
                                break;
                            }
                        }
                    }
                }
            }

            debug!("closing websocket writer stream");
            let _ = writer_stream.lock().await.close().await;
        });

        Self {
            _tx,
            writer_stream: writer_stream_clone,
        }
    }

    pub fn publisher(&self) -> SocketPublisher {
        self._tx.clone()
    }

    pub async fn close(&self) -> Result<(), RelayError> {
        Ok(self.writer_stream.lock().await.close().await?)
    }
}
