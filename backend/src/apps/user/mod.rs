use axum::{routing::{get, post}, Router};
use crate::state::AppState;

pub mod models;
pub mod handlers;
pub mod auth;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/me", get(handlers::get_me))
}
