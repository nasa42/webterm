use crate::models::terminal::{read_from_pty, write_to_pty, PtyReader, PtyWriter, Terminal};
use axum::extract::ws::{Message as WSMessage, WebSocket};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use webterm_shared::flatbuffer_helpers::{
    create_agent_to_frontend_message, create_relay_to_frontend_message,
    read_frontend_to_agent_message, read_frontend_to_relay_message,
};
use webterm_shared::generated::flatbuffers_schema::{
    AgentToFrontendMessageType, FrontendToAgentMessageType, FrontendToRelayMessageType,
    RelayToFrontendMessageType,
};
use webterm_shared::pty_output_formatter::format_pty_output;

type FrontendWriter = SplitSink<WebSocket, WSMessage>;
type FrontendReader = SplitStream<WebSocket>;
type FrontendWriterArcMutex = Arc<Mutex<FrontendWriter>>;

pub struct FrontendConnection {
    frontend_writer: FrontendWriter,
    frontend_reader: FrontendReader,
}

impl FrontendConnection {
    pub async fn new(socket: WebSocket) -> Self {
        let (frontend_writer, frontend_reader) = socket.split();
        FrontendConnection {
            frontend_writer,
            frontend_reader,
        }
    }

    pub async fn handle_connection(self) {
        let terminal = Terminal::new("/bin/bash")
            .await
            .expect("Failed to create terminal");

        let frontend_writer = Arc::new(Mutex::new(self.frontend_writer));

        let mut write_task = tokio::spawn(write_to_frontend(
            frontend_writer.clone(),
            terminal.pty_reader,
        ));

        let mut read_task = tokio::spawn({
            read_from_frontend(self.frontend_reader, frontend_writer, terminal.pty_writer)
        });

        // If any one of the tasks exit, abort the other.
        tokio::select! {
            rv_a = (&mut write_task) => {
                match rv_a {
                    Ok(_) => println!("write finished"),
                    Err(_) => println!("error in write task")
                }
                read_task.abort();
            },
            rv_b = (&mut read_task) => {
                match rv_b {
                    Ok(_) => println!("reader finished"),
                    Err(_) => println!("error in reader task")
                }
                write_task.abort();
            }
        }
    }
}

async fn read_from_frontend(
    mut frontend_reader: FrontendReader,
    mut frontend_writer: FrontendWriterArcMutex,
    mut pty_writer: PtyWriter,
) {
    loop {
        println!("\x1b[32mREADER LOOP\x1b[0m");
        let msg = { frontend_reader.next().await };

        if let Some(Ok(WSMessage::Binary(data))) = msg {
            let message = read_frontend_to_relay_message(&data);
            if let Ok((type_, data)) = message {
                match type_ {
                    FrontendToRelayMessageType::ToAgent => {
                        process_frontend_to_agent_message(
                            data,
                            &mut pty_writer,
                            &mut frontend_writer,
                        )
                        .await;
                    }
                    _ => {
                        // ignore for now
                    }
                }
            }
        }
    }
}

async fn write_to_frontend(frontend_writer: FrontendWriterArcMutex, mut pty_reader: PtyReader) {
    loop {
        println!("\x1b[32mWRITER LOOP\x1b[0m");

        let mut payload: Option<(AgentToFrontendMessageType, Vec<u8>)> = None;

        let data = read_from_pty(&mut pty_reader).await;
        match data {
            Ok(data) => {
                payload = Some((AgentToFrontendMessageType::Data, data));
            }
            Err(e) => {
                payload = Some((AgentToFrontendMessageType::Error, e.as_bytes().to_vec()));
                send_error(&frontend_writer, &e).await;
                break;
            }
        }

        if let Some(payload) = payload {
            println!("Read from console: {}##", format_pty_output(&payload.1));
            let message = create_agent_to_frontend_message(payload.0, &payload.1);
            let message =
                create_relay_to_frontend_message(RelayToFrontendMessageType::FromAgent, &message);
            let _ = frontend_writer
                .lock()
                .await
                .send(WSMessage::Binary(message))
                .await;
        }
    }
}

async fn process_frontend_to_agent_message(
    data: &[u8],
    pty_writer: &mut PtyWriter,
    frontend_writer: &mut FrontendWriterArcMutex,
) {
    let inner_message = read_frontend_to_agent_message(data);

    if let Err(_err) = inner_message {
        // ignore for now
        return;
    }

    let (type_, data) = inner_message.unwrap();

    match type_ {
        FrontendToAgentMessageType::Data => {
            println!("Received data from frontend: {}##", format_pty_output(data));
            let _ = write_to_pty(pty_writer, data).await;
        }
        FrontendToAgentMessageType::Error => {
            let _ = send_error(
                frontend_writer,
                std::str::from_utf8(data).unwrap_or_default(),
            )
            .await;
        }

        _ => {
            // ignore for now
        }
    }
}

async fn send_error(frontend_writer: &FrontendWriterArcMutex, message: &str) {
    let payload =
        create_relay_to_frontend_message(RelayToFrontendMessageType::Error, message.as_bytes());
    let _ = frontend_writer
        .lock()
        .await
        .send(WSMessage::Binary(payload))
        .await;
}
