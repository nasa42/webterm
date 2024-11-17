use crate::models::agent_error::AgentError;
use crate::models::terminal_reader::TerminalReader;
use pty_process::{Command, OwnedWritePty, Pty, Size};
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub struct Terminal {
    terminal_reader: TerminalReader,
    pty_writer: Mutex<OwnedWritePty>,
}

impl Terminal {
    pub async fn new(frontend_id: u64, command: &str) -> Result<Self, AgentError> {
        let pty = Pty::new()?;
        let mut command = Command::new(command);
        command.spawn(&pty.pts().unwrap())?;
        let (pty_reader, pty_writer) = pty.into_split();
        let pty_writer = Mutex::new(pty_writer);
        Ok(Terminal {
            terminal_reader: TerminalReader::new(frontend_id, pty_reader),
            pty_writer,
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
