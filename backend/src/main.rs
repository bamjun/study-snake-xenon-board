use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod apps;
mod state;

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load config
    let config = Config::from_env();

    // Connect to DB
    let pool = db::establish_connection(&config.database_url).await;

    // Create AppState
    let state = AppState {
        pool,
        config: Arc::new(config.clone()),
    };

    // Build Router
    let app = Router::new()
        .nest("/api/users", apps::user::router())
        .nest("/api/boards", apps::board::router())
        .with_state(state);

    // Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.server_port));
    tracing::info!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
