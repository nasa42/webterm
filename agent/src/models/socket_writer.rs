use futures::stream::SplitSink;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tracing::info;

pub type SocketPublisher = mpsc::Sender<Vec<u8>>;

pub struct SocketWriter {
    _tx: SocketPublisher,
}

impl SocketWriter {
    pub fn new(
        mut writer_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    ) -> Self {
        let (_tx, mut rx) = mpsc::channel::<Vec<u8>>(16);
        tokio::spawn(async move {
            loop {
                let received = rx.recv().await;
                match received {
                    None => {
                        info!("mpsc rx closed");
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
