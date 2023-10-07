use sea_orm::{Database, DatabaseConnection};
use std::env;

pub(crate) async fn connection() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").unwrap_or("postgres://postgres:postgres@localhost:5432/crcl".to_string());

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    conn
}
