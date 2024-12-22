use crate::models::agent_error::AgentError;
use crate::models::relay_connection::RelayConnection;
use futures::stream::SplitStream;
use futures::StreamExt;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::{error, info};
use webterm_core::models::reader_socket_error::ReaderSocketError;

pub type SocketSubscriber = broadcast::Receiver<Result<Option<Vec<u8>>, ReaderSocketError>>;

pub struct SocketReader {
    _tx: broadcast::Sender<Result<Option<Vec<u8>>, ReaderSocketError>>,
}

impl SocketReader {
    pub fn new(
        mut reader_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        rc: Arc<RelayConnection>,
    ) -> Self {
        let (_tx, _rx) = broadcast::channel::<Result<Option<Vec<u8>>, ReaderSocketError>>(16);
        let tx = _tx.clone();
        let rc_clone = rc.clone();

        tokio::spawn(async move {
            loop {
                if let Some(received) = reader_stream.next().await {
                    let received = match received {
                        Ok(Message::Binary(received)) => Ok(Some(received)),
                        Ok(Message::Close(_)) => {
                            rc_clone.disconnect().await;
                            break;
                        }
                        Ok(Message::Ping(_)) =>
                        /* TODO: handle ping */
                        {
                            Ok(None)
                        }
                        Ok(Message::Pong(_)) =>
                        /* TODO: handle pong */
                        {
                            Ok(None)
                        }
                        Ok(Message::Text(_)) =>
                        /* TODO: handle text */
                        {
                            Ok(None)
                        }
                        Ok(Message::Frame(_)) =>
                        /* TODO: handle text */
                        {
                            Ok(None)
                        }
                        Err(error) => {
                            error!("Error receiving message from stream: {}", error);
                            rc_clone
                                .disconnect_with_error(AgentError::RuntimeError(error.to_string()))
                                .await;
                            break;
                        }
                    };

                    let _ = tx.send(received);
                } else {
                    info!("Reader stream closed");
                    rc_clone.disconnect().await;
                    break;
                }
            }
        });
        Self { _tx }
    }

    pub fn subscriber(&self) -> SocketSubscriber {
        self._tx.subscribe()
    }
}
