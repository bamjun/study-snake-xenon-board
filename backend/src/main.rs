use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod db;
mod apps;
mod state;

use config::Config;
use state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        apps::user::handlers::register,
        apps::user::handlers::login,
        apps::user::handlers::get_me,
        apps::board::handlers::list_posts,
        apps::board::handlers::create_post,
        apps::board::handlers::get_post,
    ),
    components(
        schemas(
            apps::user::models::User,
            apps::user::models::CreateUserRequest,
            apps::user::models::LoginRequest,
            apps::user::models::AuthResponse,
            apps::board::models::Post,
            apps::board::models::CreatePostRequest,
            apps::board::models::UpdatePostRequest,
        )
    ),
    tags(
        (name = "backend", description = "Backend API")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.security_schemes.insert(
                "bearer_auth".to_string(),
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
        openapi.security = Some(vec![utoipa::openapi::security::SecurityRequirement::new(
            "bearer_auth",
            Vec::<String>::new(),
        )]);
    }
}

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

    // Build API Router (with state)
    let api_router = Router::new()
        .nest("/api/users", apps::user::router())
        .nest("/api/boards", apps::board::router())
        .with_state(state);

    // CORS Layer
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Build Main Router (merging Swagger and API)
    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api_router)
        .layer(cors);

    // Start Server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("🚀 Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
