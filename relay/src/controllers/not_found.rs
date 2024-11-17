use askama_axum::Template;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "404.html")]
struct HTMLTemplate<'a> {
    browser_title: &'a str,
    project_url: &'a str,
}

pub async fn handler() -> impl IntoResponse {
    let template = HTMLTemplate {
        browser_title: "Web Terminal Relay",
        project_url: "https://github.com/nasa42/webterm",
    };

    (StatusCode::NOT_FOUND, template)
}
