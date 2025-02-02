use crate::helpers::render_response::render_response;
use askama::Template;
use axum::http::StatusCode;
use axum::response::Response;

#[derive(Template)]
#[template(path = "404.html")]
struct HTMLTemplate<'a> {
    browser_title: &'a str,
    project_url: &'a str,
}

pub async fn handler() -> Response {
    let template = HTMLTemplate {
        browser_title: "Webterm Relay",
        project_url: "https://github.com/nasa42/webterm",
    };

    render_response(StatusCode::NOT_FOUND, &template)
}
