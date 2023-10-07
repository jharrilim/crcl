use axum::{
    routing::{get, post},
    Router,
};
use entity;
use sea_orm::DatabaseConnection;

use crate::controllers;

#[derive(Clone)]
pub(crate) struct AppState {
    pub conn: DatabaseConnection,
}

pub(crate) fn router(conn: DatabaseConnection) -> Router {
    let state = AppState { conn };

    Router::new()
        .route("/", get(controllers::index::index))
        .route("/greet/:name", get(controllers::greet::greet))
        .route("/profile", get(controllers::user::index))
        .route("/u/:name", get(controllers::user::show))
        .route("/auth/login", get(controllers::auth::login))
        .route("/auth/login", post(controllers::auth::login_post))
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .with_state(state)
}
