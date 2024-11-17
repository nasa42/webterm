use crate::models::frontend_connection::FrontendConnection;
use std::sync::Arc;

pub async fn perform_handshake(_frontend_connection: Arc<FrontendConnection>) -> Result<(), ()> {
    Ok(())
}
