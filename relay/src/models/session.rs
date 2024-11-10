use crate::models::agent_connection::AgentConnection;
use crate::models::frontend_connection::FrontendConnection;

pub struct Session {
    frontend: FrontendConnection,
    agent: AgentConnection,
}
