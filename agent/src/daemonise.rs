use crate::models::panic_error::PanicError;
use std::fs::OpenOptions;

pub fn daemonise() -> Result<(), PanicError> {
    use daemonize::Daemonize;

    let log_path = "/tmp/webterm-agent.log";

    let stdout = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .map_err(|_| {
            PanicError::RuntimeError(format!("Could not create stdout log file: {}", log_path))
        })?;

    let stderr = stdout.try_clone().map_err(|_| {
        PanicError::RuntimeError(format!("Could not create stderr log file: {}", log_path))
    })?;

    let daemonize = Daemonize::new()
        .pid_file("/tmp/webterm-agent.pid")
        .stdout(stdout)
        .stderr(stderr);

    println!("Running in background, logging to {}", log_path);

    match daemonize.start() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to run in background: {}", e);
            Err(PanicError::RuntimeError(e.to_string()))
        }
    }
}
