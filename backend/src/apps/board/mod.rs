use axum::{routing::get, Router};
use crate::state::AppState;

pub mod models;
pub mod handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::list_posts).post(handlers::create_post))
        .route("/:id", get(handlers::get_post))
}
