use crate::models::agent_error::AgentError;
use crate::models::pty_activity_reader::PtyActivityReader;
use pty_process::{Command, OwnedWritePty, Pty, Size};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use webterm_core::types::ActivityId;

pub struct PtyActivity {
    // required to keep the reader alive
    _pty_reader: PtyActivityReader,
    pty_writer: Mutex<OwnedWritePty>,
}

impl PtyActivity {
    pub async fn new(activity_id: ActivityId, command: &str) -> Result<Self, AgentError> {
        let pty = Pty::new()?;
        let mut command = Command::new(command);
        command.env("TERM", "xterm-256color");
        command.spawn(&pty.pts().unwrap())?;
        let (pty_reader, pty_writer) = pty.into_split();
        let pty_writer = Mutex::new(pty_writer);
        Ok(PtyActivity {
            pty_writer,
            _pty_reader: PtyActivityReader::new(activity_id, pty_reader),
        })
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> Result<(), AgentError> {
        self.pty_writer.lock().await.resize(Size::new(cols, rows))?;
        Ok(())
    }

    pub async fn write(&self, data: &[u8]) -> Result<(), AgentError> {
        self.pty_writer.lock().await.write_all(data).await?;
        Ok(())
    }

    pub async fn shutdown(&self) -> Result<(), AgentError> {
        self.pty_writer.lock().await.shutdown().await?;
        Ok(())
    }
}
