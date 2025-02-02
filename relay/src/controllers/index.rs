use crate::helpers::render_response::render_response;
use askama::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    browser_title: &'a str,
    project_url: &'a str,
}

pub async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {
        browser_title: "Webterm Relay",
        project_url: "https://github.com/nasa42/webterm",
    };

    render_response(StatusCode::OK, &template)
}
