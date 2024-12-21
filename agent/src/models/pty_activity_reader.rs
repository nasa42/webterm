use crate::models::activity::Activity;
use pty_process::OwnedReadPty;
use std::sync::{Arc, OnceLock};
use tokio::io::AsyncReadExt;
use tokio::sync::{broadcast, mpsc, Mutex};
use tracing::debug;
use tracing::info;
use webterm_core::pty_output_formatter::format_pty_output;
use webterm_core::serialisers::talk_v1::terminal_output_builder::{
    ActivityOutputBlob, TerminalOutputBuilder,
};
use webterm_core::types::{ActivityId, SessionId};

const BUFFER_SIZE: usize = 10240;

pub type TerminalSubscriber = broadcast::Receiver<Vec<u8>>;

type ChannelType = (
    mpsc::Sender<PtyActivityReaderPayload>,
    Mutex<mpsc::Receiver<PtyActivityReaderPayload>>,
);

pub struct PtyActivityReaderPayload {
    pub(crate) activity_id: ActivityId,
    pub(crate) data: Vec<u8>,
}

impl PtyActivityReaderPayload {
    pub fn to_fb_output(&self) -> ActivityOutputBlob {
        let builder = TerminalOutputBuilder::new();
        builder.build_output(&self.data).to_flatbuffers()
    }
}

pub struct PtyActivityReader {}

impl PtyActivityReader {
    pub fn channel() -> &'static ChannelType {
        static CHANNEL: OnceLock<ChannelType> = OnceLock::new();
        CHANNEL.get_or_init(|| {
            let (tx, rx) = mpsc::channel::<PtyActivityReaderPayload>(1024);
            (tx, Mutex::new(rx))
        })
    }

    pub fn sender() -> mpsc::Sender<PtyActivityReaderPayload> {
        Self::channel().0.clone()
    }

    pub fn receiver() -> &'static Mutex<mpsc::Receiver<PtyActivityReaderPayload>> {
        &Self::channel().1
    }

    pub fn new(activity_id: ActivityId, mut reader_stream: OwnedReadPty) -> Self {
        let sender = Self::sender();
        tokio::spawn(async move {
            debug!("starting new terminal reader stream");
            loop {
                let mut buf = [0u8; BUFFER_SIZE];
                if let Ok(length) = reader_stream.read(&mut buf).await {
                    // debug!(
                    //     "read from reader stream: {:?}",
                    //     format_pty_output(&buf[..length])
                    // );
                    sender
                        .send(PtyActivityReaderPayload {
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
