mod controllers;

use axum::response::IntoResponse;
use axum::{
    routing::{any, get},
    Router, ServiceExt,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(controllers::index::handler))
        .route("/up", get(controllers::up::handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
