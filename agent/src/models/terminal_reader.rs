use pty_process::OwnedReadPty;
use std::sync::OnceLock;
use tokio::io::AsyncReadExt;
use tokio::sync::{broadcast, mpsc, Mutex};
use tracing::debug;
use tracing::info;
use webterm_shared::pty_output_formatter::format_pty_output;
use webterm_shared::types::ActivityId;

pub type TerminalSubscriber = broadcast::Receiver<Vec<u8>>;

type ChannelType = (
    mpsc::Sender<TerminalReaderPayload>,
    Mutex<mpsc::Receiver<TerminalReaderPayload>>,
);

#[derive(Debug, Clone)]
pub struct TerminalReaderPayload {
    pub(crate) activity_id: ActivityId,
    pub(crate) data: Vec<u8>,
}

pub struct TerminalReader {}

impl TerminalReader {
    pub fn channel() -> &'static ChannelType {
        static CHANNEL: OnceLock<ChannelType> = OnceLock::new();
        CHANNEL.get_or_init(|| {
            let (tx, rx) = mpsc::channel::<TerminalReaderPayload>(1024);
            (tx, Mutex::new(rx))
        })
    }

    pub fn sender() -> mpsc::Sender<TerminalReaderPayload> {
        Self::channel().0.clone()
    }

    pub fn receiver() -> &'static Mutex<mpsc::Receiver<TerminalReaderPayload>> {
        &Self::channel().1
    }

    pub fn new(activity_id: ActivityId, mut reader_stream: OwnedReadPty) -> Self {
        let sender = Self::sender();
        tokio::spawn(async move {
            debug!("starting new terminal reader stream");
            loop {
                let mut buf = [0u8; 1024];
                if let Ok(length) = reader_stream.read(&mut buf).await {
                    debug!(
                        "read from reader stream: {:?}",
                        format_pty_output(&buf[..length])
                    );
                    sender
                        .send(TerminalReaderPayload {
                            activity_id,
                            data: buf[..length].to_vec(),
                        })
                        .await
                        .expect("this shouldn't fail");
                } else {
                    info!("Reader stream closed");
                    break;
                }
            }
        });

        Self {}
    }
}
