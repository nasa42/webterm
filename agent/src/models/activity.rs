use crate::models::activity_registry::ActivityRegistry;
use crate::models::agent_error::AgentError;
use crate::models::pty_activity::PtyActivity;
use crate::models::session::Session;
use crate::models::session_registry::SessionRegistry;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use webterm_core::generated::flatbuffers_schema::talk_v1::activity::{PtyInput, PtyInputRoot};
use webterm_core::serialisers::talk_v1::terminal_output_builder::ActivityInputBlob;
use webterm_core::types::{ActivityId, SessionId};

// in future, manage more activities like a "file browser"
pub enum ActivityType {
    Pty,
}

pub struct Activity {
    activity_id: ActivityId,
    activity_type: ActivityType,
    terminal: Option<PtyActivity>,
    parent_session_id: SessionId,
}

impl PartialEq for Activity {
    fn eq(&self, other: &Self) -> bool {
        self.activity_id == other.activity_id
    }
}

impl Activity {
    pub async fn create_pty(session_id: SessionId) -> Result<Arc<Activity>, AgentError> {
        let activity_id = ActivityRegistry::next_activity_id();
        let terminal = PtyActivity::new(activity_id, "/bin/bash").await?;
        let record = Arc::new(Self {
            activity_id,
            terminal: Some(terminal),
            activity_type: ActivityType::Pty,
            parent_session_id: session_id,
        });

        ActivityRegistry::register(record.clone()).await?;
        Ok(record)
    }

    pub fn activity_id(&self) -> ActivityId {
        self.activity_id
    }

    pub async fn parent_session(&self) -> Result<Arc<Mutex<Session>>, AgentError> {
        SessionRegistry::find(self.parent_session_id).await
    }

    pub async fn receive_input(&self, payload: ActivityInputBlob) -> Result<(), AgentError> {
        match self.activity_type {
            ActivityType::Pty => {
                let input = flatbuffers::root::<PtyInputRoot>(&payload.0)?;
                match input.payload_type() {
                    PtyInput::UserInput => {
                        self.terminal
                            .as_ref()
                            .ok_or(AgentError::RuntimeError(
                                "Terminal not initialised".to_string(),
                            ))?
                            .write(
                                input
                                    .payload_as_user_input()
                                    .ok_or(AgentError::FBParseError(
                                        "Expected user input vector".to_string(),
                                    ))?
                                    .data()
                                    .ok_or(AgentError::FBParseError(
                                        "Expected user input data".to_string(),
                                    ))?
                                    .bytes(),
                            )
                            .await?;
                        Ok(())
                    }
                    PtyInput::Resize => {
                        info!("Received resize input: {:?}", input.payload_as_resize());
                        let resize_data = input
                            .payload_as_resize()
                            .ok_or(AgentError::FBParseError("Expected resize data".to_string()))?;
                        self.terminal
                            .as_ref()
                            .ok_or(AgentError::RuntimeError(
                                "Terminal not initialised".to_string(),
                            ))?
                            .resize(resize_data.cols(), resize_data.rows())
                            .await?;
                        Ok(())
                    }

                    _ => Err(AgentError::FBParseError(format!(
                        "Unknown terminal input type: {:?}",
                        input.payload_type()
                    ))),
                }
            }
        }
    }
}
