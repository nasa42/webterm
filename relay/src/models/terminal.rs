use pty_process::{Command, OwnedReadPty, OwnedWritePty, Pty};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub type PtyWriter = OwnedWritePty;
pub type PtyReader = OwnedReadPty;

pub struct Terminal {
    pub(crate) pty_writer: PtyWriter,
    pub(crate) pty_reader: PtyReader,
}

impl Terminal {
    pub async fn new(command: &str) -> Result<Self, String> {
        let pty = Pty::new().map_err(|e| e.to_string())?;
        spawn_child(&pty, command).await?;
        let (pty_reader, pty_writer) = pty.into_split();
        Ok(Terminal {
            pty_writer,
            pty_reader,
        })
    }
}

pub async fn write_to_pty(pty_writer: &mut PtyWriter, data: &[u8]) -> Result<(), String> {
    pty_writer.write_all(data).await.map_err(|e| e.to_string())
}

pub async fn read_from_pty(pty_reader: &mut PtyReader) -> Result<Vec<u8>, String> {
    let mut buf = [0u8; 1024];
    let len = pty_reader.read(&mut buf).await.map_err(|e| e.to_string())?;
    Ok(buf[..len].to_vec())
}

pub async fn spawn_child(pty: &Pty, command: &str) -> Result<(), String> {
    let mut command = Command::new(command);
    command
        .spawn(&pty.pts().unwrap())
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use webterm_shared::pty_output_formatter::format_pty_output;

    impl Terminal {
        pub async fn with_bin_sh() -> Result<Self, String> {
            Self::new("/bin/sh").await
        }
    }

    #[tokio::test]
    async fn test_terminal_new() {
        let terminal = Terminal::with_bin_sh().await;
        assert!(terminal.is_ok());
    }

    #[tokio::test]
    async fn test_write_to_pty() {
        let mut terminal = Terminal::with_bin_sh().await.unwrap();
        let data = b"test data";
        let result = write_to_pty(&mut terminal.pty_writer, data).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_read_from_pty() {
        let mut terminal = Terminal::with_bin_sh().await.unwrap();
        let data = b"test data";
        write_to_pty(&mut terminal.pty_writer, data).await.unwrap();
        let result = read_from_pty(&mut terminal.pty_reader).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data.to_vec());
    }

    #[tokio::test]
    async fn test_echo_command() {
        let mut terminal = Terminal::with_bin_sh()
            .await
            .expect("Failed to create terminal");

        // Read any initial data from the shell startup
        let _initial_output = read_from_pty(&mut terminal.pty_reader)
            .await
            .expect("Failed to read initial data");

        // Write the echo command to the pty
        write_to_pty(&mut terminal.pty_writer, b"echo hello\n")
            .await
            .expect("Failed to write to pty");

        let mut output = read_from_pty(&mut terminal.pty_reader)
            .await
            .expect("Failed to read expected output");

        // read again for the output of the echo command
        output.extend(
            read_from_pty(&mut terminal.pty_reader)
                .await
                .expect("Failed to read expected output"),
        );

        assert_eq!(
            format_pty_output(&output),
            "echo hello\r\nhello\r\n$ ",
            "Output doesn't match: {}",
            format_pty_output(&output)
        );
    }

    #[tokio::test]
    async fn test_spawn_child() {
        let pty = Pty::new().unwrap();
        let result = spawn_child(&pty, "/bin/sh").await;
        assert!(result.is_ok());
    }
}
