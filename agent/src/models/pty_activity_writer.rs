use pty_process::OwnedWritePty;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, Mutex};
use tracing::info;

pub type TerminalPublisher = mpsc::Sender<Vec<u8>>;

pub struct PtyActivityWriter {
    _tx: TerminalPublisher,
}

impl PtyActivityWriter {
    pub fn new(writer_stream: Arc<Mutex<OwnedWritePty>>) -> Self {
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
                        let result = writer_stream.lock().await.write_all(&message).await;
                        match result {
                            Ok(_) => {
                                // continue
                            }
                            Err(error) => {
                                info!("Error sending message to writer stream: {}", error);
                                break;
                            }
                        }
                    }
                }
            }
        });
        Self { _tx }
    }

    pub fn publisher(&self) -> TerminalPublisher {
        self._tx.clone()
    }
}
