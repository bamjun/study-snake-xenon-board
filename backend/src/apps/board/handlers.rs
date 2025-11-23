use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use uuid::Uuid;
use crate::state::AppState;
use super::models::{CreatePostRequest, Post};


#[utoipa::path(
    get,
    path = "/api/boards",
    responses(
        (status = 200, description = "List all posts", body = [Post])
    )
)]
pub async fn list_posts(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&state.pool)
        .await;

    match posts {
        Ok(posts) => Json(posts).into_response(),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/boards",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created", body = Post)
    )
)]
pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (id, title, content, author_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.author_id)
    .fetch_one(&state.pool)
    .await;

    match post {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/boards/{id}",
    params(
        ("id" = Uuid, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Get post by ID", body = Post),
        (status = 404, description = "Post not found")
    )
)]
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await;

    match post {
        Ok(Some(post)) => Json(post).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Post not found").into_response(),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}
