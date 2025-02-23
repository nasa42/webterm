use crate::models::agent_connection::AgentConnection;
use crate::models::socket_connection::SocketConnection;
use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitStream;
use futures::StreamExt;
use std::sync::{Arc, Once};
use tokio::sync::{broadcast, Notify};
use tracing::{error, info};
use webterm_core::models::reader_socket_error::ReaderSocketError;

pub type SocketSubscriber = broadcast::Receiver<Result<Option<Vec<u8>>, ReaderSocketError>>;

pub struct SocketReader {
    _tx: broadcast::Sender<Result<Option<Vec<u8>>, ReaderSocketError>>,
}

impl SocketReader {
    pub fn new(mut reader_stream: SplitStream<WebSocket>, close_notifier: Arc<Notify>) -> Self {
        let (_tx, _rx) = broadcast::channel::<Result<Option<Vec<u8>>, ReaderSocketError>>(16);
        let tx = _tx.clone();
        tokio::spawn(async move {
            loop {
                if let Some(received) = reader_stream.next().await {
                    let received = match received {
                        Ok(Message::Binary(received)) => Ok(Some(received)),
                        Ok(Message::Close(_)) => Err(ReaderSocketError::SocketClosed),
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
                        Err(error) => {
                            error!("Error receiving message from stream: {}", error);
                            break;
                        }
                    };

                    let _ = tx.send(received.map(|opt| opt.map(|bytes| bytes.to_vec())));
                } else {
                    info!("Reader stream closed");
                    break;
                }
            }

            close_notifier.notify_waiters();
        });
        Self { _tx }
    }

    pub fn subscriber(&self) -> SocketSubscriber {
        self._tx.subscribe()
    }
}
