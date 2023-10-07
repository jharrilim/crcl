use askama::Template;

use axum::{extract, response::IntoResponse};

use crate::templating::HtmlTemplate;

pub(crate) async fn greet(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}
