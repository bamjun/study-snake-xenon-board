use sqlx::postgres::{PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn establish_connection(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
        .expect("Failed to connect to Postgres")
}
