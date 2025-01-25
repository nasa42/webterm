use crate::models::agent_error::AgentError;
use crate::models::connection_manager::ConnectionManager;
use futures::stream::SplitSink;
use futures::SinkExt;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::{Bytes, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::info;

pub type SocketPublisher = mpsc::Sender<Bytes>;

pub struct SocketWriter {
    _tx: SocketPublisher,
}

impl SocketWriter {
    pub fn new(
        mut writer_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        cm: Arc<ConnectionManager>,
    ) -> Self {
        let (_tx, mut rx) = mpsc::channel::<Bytes>(16);

        tokio::spawn(async move {
            loop {
                let received = rx.recv().await;
                match received {
                    None => {
                        info!("mpsc rx closed");
                        cm.disconnect().await;
                        break;
                    }
                    Some(message) => {
                        let result = writer_stream.send(Message::Binary(message)).await;
                        match result {
                            Ok(_) => {
                                // continue
                            }
                            Err(error) => {
                                info!("Error sending message to writer stream: {:?}", error);
                                cm.disconnect_with_error(AgentError::RuntimeError(
                                    error.to_string(),
                                ))
                                .await;
                                break;
                            }
                        }
                    }
                }
            }
        });
        Self { _tx }
    }

    pub fn publisher(&self) -> SocketPublisher {
        self._tx.clone()
    }
}
