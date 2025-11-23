use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};

use crate::state::AppState;
use super::models::{CreateUserRequest, LoginRequest, User, AuthResponse};
use super::auth::{hash_password, verify_password, create_jwt};

#[utoipa::path(
    post,
    path = "/api/users/register",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    // 1. Hash password
    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing failed").into_response(),
    };

    // 2. Insert into DB
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, password_hash, email, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING id, username, password_hash, email, created_at, updated_at
        "#
    )
    .bind(uuid::Uuid::new_v4())
    .bind(payload.username)
    .bind(password_hash)
    .bind(payload.email)
    .fetch_one(&state.pool)
    .await;

    match user {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => {
            // Handle unique constraint violation etc.
            tracing::error!("Database error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/users/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // 1. Find user
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(payload.username)
    .fetch_optional(&state.pool)
    .await;

    match user {
        Ok(Some(user)) => {
            // 2. Verify password
            match verify_password(&payload.password, &user.password_hash) {
                Ok(true) => {
                    // 3. Generate JWT
                    match create_jwt(user.id, &state.config) {
                        Ok(token) => Json(AuthResponse { token }).into_response(),
                        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed").into_response(),
                    }
                }
                _ => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
            }
        }
        Ok(None) => (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response(),
        Err(e) => {
            tracing::error!("Database error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}
