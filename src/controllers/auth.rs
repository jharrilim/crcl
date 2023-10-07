use askama::Template;
use axum::{extract, response::Redirect};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::templating::HtmlTemplate;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate {}
pub(crate) async fn login() -> impl IntoResponse {
    HtmlTemplate(LoginTemplate {})
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct LoginFormData {
    username: String,
    password: String,
}
pub(crate) async fn login_post(
    extract::Form(login_form): extract::Form<LoginFormData>,
) -> impl IntoResponse {
    tracing::debug!("Login form submitted: {:?}", login_form);

    Redirect::to("/profile")
}
