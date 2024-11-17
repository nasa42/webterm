use askama_axum::IntoResponse;
use axum::body::Bytes;

#[axum::debug_handler]
pub async fn frontend_handler(_body: Bytes) -> impl IntoResponse {
    // purpose
    // ask relay if a server_id exists
    // ask frontend to perform proof of work
    // create an "auth_nonce" which can be used to establish a websocket connection
    ""
}

#[axum::debug_handler]
pub async fn agent_handler(_body: Bytes) -> impl IntoResponse {
    // purpose
    // ask relay if a server_id exists
    // ask frontend to perform proof of work
    // create an "auth_nonce" which can be used to establish a websocket connection
    ""
}
