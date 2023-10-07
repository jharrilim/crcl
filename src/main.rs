

use crate::{router::router, config::db};
use std::{env, net::SocketAddr, str::FromStr};

pub(crate) mod config;
pub(crate) mod controllers;
mod router;
pub(crate) mod templating;

use migration::{Migrator, MigratorTrait};

async fn init() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "debug");
    dotenvy::dotenv().ok();

    config::traces::init();

    let conn = db::connection().await;

    Migrator::up(&conn, None).await?;

    let app = router(conn);

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let server_url = format!("{host}:{port}");

    let addr = SocketAddr::from_str(&server_url).expect("Invalid server URL");

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(config::shutdown::signal())
        .await
        .unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init().await
}
