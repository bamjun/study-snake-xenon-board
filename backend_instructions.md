# Backend 구축 가이드

이 문서는 현재 `backend` 디렉토리의 구성을 처음부터 다시 구축하기 위한 단계별 가이드입니다.

## 1. 프로젝트 초기화

프로젝트 루트에서 Rust 프로젝트를 생성합니다.

```bash
cargo new backend
cd backend
```

## 2. 의존성 설정 (`Cargo.toml`)

`Cargo.toml` 파일을 열고 다음 내용을 입력합니다. 주요 라이브러리는 Axum(웹 프레임워크), SQLx(데이터베이스), Tokio(비동기 런타임)입니다.

```toml
[package]
name = "backend"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7.9"
tokio = { version = "1.43.0", features = ["full"] }
sqlx = { version = "0.8.3", features = ["postgres", "runtime-tokio", "tls-native-tls", "macros", "chrono", "uuid"] }
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.138"
dotenvy = "0.15.7"
tracing = "0.1.41"
tracing-subscriber = { version = "0.3.19", features = ["env-filter"] }
jsonwebtoken = "9.3.1"
argon2 = "0.5.3"
chrono = { version = "0.4.39", features = ["serde"] }
uuid = { version = "1.12.1", features = ["serde", "v4"] }
tower-http = { version = "0.5.2", features = ["cors", "trace"] }
utoipa = { version = "5.2.0", features = ["axum_extras", "uuid", "chrono"] }
utoipa-swagger-ui = { version = "8.0.0", features = ["axum"] }
```

## 3. 환경 변수 설정 (`.env`)

`backend` 디렉토리 내에 `.env` 파일을 생성합니다.

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/study_snake_db
JWT_SECRET=your_super_secret_jwt_key
PORT=3000
RUST_LOG=backend=debug,tower_http=debug
```

## 4. 데이터베이스 설정

### 4.1 PostgreSQL 실행 (Docker)
프로젝트 루트의 `docker-compose-dev.yml`을 사용하거나 아래 명령어로 실행합니다.

```bash
docker run --name postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=study_snake_db -p 5432:5432 -d postgres:15
```

### 4.2 SQLx CLI 설치 및 마이그레이션
```bash
# sqlx-cli 설치 (없을 경우)
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# 데이터베이스 생성
sqlx database create

# 마이그레이션 파일 생성
sqlx migrate add create_users_and_posts_table
```

생성된 `migrations/YYYYMMDDHHMMSS_create_users_and_posts_table.sql` 파일에 다음 SQL을 작성합니다:

```sql
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS posts (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

마이그레이션 적용:
```bash
sqlx migrate run
```

## 5. 소스 코드 구조 생성

다음과 같이 디렉토리 구조를 만듭니다.

```bash
mkdir -p src/apps/user
mkdir -p src/apps/board
```

## 6. 핵심 모듈 구현

### `src/config.rs`
환경 설정을 로드합니다.

```rust
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let server_port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        Self {
            database_url,
            jwt_secret,
            server_port,
        }
    }
}
```

### `src/db.rs`
DB 연결 풀을 관리합니다.

```rust
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
```

### `src/state.rs`
애플리케이션 전역 상태를 정의합니다.

```rust
use sqlx::PgPool;
use crate::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Arc<Config>,
}
```

### `src/apps/mod.rs`
앱 모듈을 등록합니다.

```rust
pub mod user;
pub mod board;
```

## 7. User 앱 구현 (`src/apps/user/`)

### `src/apps/user/models.rs`
데이터 모델과 DTO를 정의합니다.

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
}
```

### `src/apps/user/auth.rs`
JWT 생성, 비밀번호 해싱, 인증 미들웨어(Extractor)를 구현합니다.

```rust
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use uuid::Uuid;
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub fn create_jwt(user_id: Uuid, config: &Config) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 60 * 60 * 24;

    let claims = Claims { sub: user_id.to_string(), exp: expiration };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt_secret.as_bytes()))
}

use axum::{async_trait, extract::FromRequestParts, http::{header::AUTHORIZATION, request::Parts, StatusCode}};
use jsonwebtoken::{decode, Validation, DecodingKey};
use crate::state::AppState;
use super::models::User;

pub struct AuthUser(pub User);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;

        let token = if auth_header.starts_with("Bearer ") { &auth_header[7..] } else {
            return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header format".to_string()));
        };

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid user ID".to_string()))?;
        
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&state.pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or((StatusCode::UNAUTHORIZED, "User not found".to_string()))?;

        Ok(AuthUser(user))
    }
}
```

### `src/apps/user/handlers.rs`
실제 비즈니스 로직을 처리합니다.

```rust
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Json}};
use crate::state::AppState;
use super::models::{CreateUserRequest, LoginRequest, User, AuthResponse};
use super::auth::{hash_password, verify_password, create_jwt, AuthUser};

#[utoipa::path(
    post, path = "/api/users/register", request_body = CreateUserRequest,
    responses((status = 201, description = "Created", body = User))
)]
pub async fn register(State(state): State<AppState>, Json(payload): Json<CreateUserRequest>) -> impl IntoResponse {
    let password_hash = hash_password(&payload.password).unwrap();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, password_hash, email) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(uuid::Uuid::new_v4()).bind(payload.username).bind(password_hash).bind(payload.email)
    .fetch_one(&state.pool).await;

    match user {
        Ok(u) => (StatusCode::CREATED, Json(u)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creating user").into_response(),
    }
}

#[utoipa::path(
    post, path = "/api/users/login", request_body = LoginRequest,
    responses((status = 200, body = AuthResponse))
)]
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(payload.username).fetch_optional(&state.pool).await.unwrap();

    if let Some(user) = user {
        if verify_password(&payload.password, &user.password_hash).unwrap_or(false) {
            let token = create_jwt(user.id, &state.config).unwrap();
            return Json(AuthResponse { token }).into_response();
        }
    }
    (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
}

#[utoipa::path(
    get, path = "/api/users/me", responses((status = 200, body = User)), security(("bearer_auth" = []))
)]
pub async fn get_me(AuthUser(user): AuthUser) -> impl IntoResponse {
    Json(user)
}
```

### `src/apps/user/mod.rs`
라우터를 정의합니다.

```rust
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
```

## 8. 메인 엔트리 포인트 (`src/main.rs`)

모든 컴포넌트를 조립합니다.

```rust
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
        // apps::board::handlers::... (Board 관련 핸들러 추가 필요)
    ),
    components(schemas(
        apps::user::models::User,
        apps::user::models::CreateUserRequest,
        apps::user::models::LoginRequest,
        apps::user::models::AuthResponse,
    )),
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
        openapi.security = Some(vec![utoipa::openapi::security::SecurityRequirement::new("bearer_auth", vec![])]);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(std::env::var("RUST_LOG").unwrap_or("debug".into())))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let pool = db::establish_connection(&config.database_url).await;
    let state = AppState { pool, config: Arc::new(config.clone()) };

    let api_router = Router::new()
        .nest("/api/users", apps::user::router())
        // .nest("/api/boards", apps::board::router()) // Board 라우터 추가 필요
        .with_state(state);

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(api_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Server running at http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## 9. 실행

```bash
cargo run
```

서버가 시작되면 `http://localhost:3000/docs`에서 Swagger UI를 확인할 수 있습니다.
