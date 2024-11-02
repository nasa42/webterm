mod controllers;
mod frontend_proxy;

use axum::response::IntoResponse;
use axum::{
    routing::{any, get},
    Router, ServiceExt,
};
use std::env;

#[tokio::main]
async fn main() {
    let mut app = Router::new().route("/b/up", get(controllers::up::handler));
    let app_env = env::var("APP_ENV").unwrap_or("development".to_string());

    if app_env == "development" {
        app = app.nest_service(
            "/",
            any(frontend_proxy::proxy_handler).with_state(frontend_proxy::get_client()),
        );
    } else {
        app = app.nest_service("/", frontend_proxy::static_handler());
    }

    let app = app;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
