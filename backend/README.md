# Axum Backend Project

Rust의 **Axum** 프레임워크를 사용하여 **Django**와 유사한 구조(App 기반)로 설계된 백엔드 API 서버입니다.
**PostgreSQL** 데이터베이스와 **JWT** 인증을 포함하고 있습니다.

## 📂 프로젝트 구조 분석 (Project Structure)

이 프로젝트는 유지보수와 확장성을 위해 Django의 "App" 개념을 차용하여 기능별로 모듈을 분리했습니다.

```text
backend/
├── Cargo.toml          # 📦 프로젝트 명세서 (Python의 requirements.txt + setup.py)
│                       # 사용되는 라이브러리(crate)와 버전을 정의합니다.
├── .env                # 🔒 환경 변수 (DB 주소, JWT 비밀키 등 민감 정보)
├── schema.sql          # 🗄️ 데이터베이스 스키마 (테이블 생성 SQL)
├── src/
│   ├── main.rs         # 🏁 프로그램 진입점 (Entry Point)
│   │                   # 서버를 시작하고, 미들웨어와 각 앱의 라우터를 하나로 통합합니다.
│   ├── config.rs       # ⚙️ 설정 관리
│   │                   # .env 파일에서 환경 변수를 읽어와 구조체로 관리합니다.
│   ├── db.rs           # 🔌 DB 연결
│   │                   # PostgreSQL 연결 풀(Connection Pool)을 생성하고 설정합니다.
│   ├── state.rs        # 🌐 전역 상태 (Global State)
│   │                   # DB Pool, Config 등 애플리케이션 전체에서 공유해야 할 데이터를 담습니다.
│   └── apps/           # 🧩 기능 모듈 (Django의 Apps)
│       ├── mod.rs      # 앱 모듈 등록 파일
│       ├── user/       # [User 앱] 사용자 관련 기능
│       │   ├── mod.rs      # 라우터 (URL 경로 정의)
│       │   ├── models.rs   # 데이터 모델 (DB 테이블 구조 & API 요청/응답 구조)
│       │   ├── handlers.rs # 핸들러 (비즈니스 로직, Controller/View 역할)
│       │   └── auth.rs     # 인증 유틸리티 (JWT 생성/검증, 비밀번호 해싱)
│       └── board/      # [Board 앱] 게시판 관련 기능
│           ├── mod.rs      # 라우터
│           ├── models.rs   # 데이터 모델
│           └── handlers.rs # 핸들러
```

---

## 🚀 프로젝트 시작 가이드 (Getting Started)

이 프로젝트를 처음부터 세팅하고 실행하는 과정을 단계별로 설명합니다.

### 1. 프로젝트 초기화 (Initialization)
Rust 프로젝트를 새로 시작할 때 사용하는 명령어입니다. (이 프로젝트는 이미 생성되어 있으므로 생략 가능하지만, 과정 이해를 위해 적어둡니다.)
```bash
# 새로운 바이너리 프로젝트 생성
cargo init backend
cd backend
```

```bash
# sqlx migrate add user_table 작성
sqlx migrate add user_table 
# sqlx migrate run
sqlx migrate run
```

### 2. 의존성 추가 (Dependencies)
`Cargo.toml` 파일에 필요한 라이브러리를 추가합니다.
- **axum**: 웹 프레임워크
- **tokio**: 비동기 런타임
- **sqlx**: 데이터베이스 ORM/Query Builder
- **serde**: 데이터 직렬화/역직렬화 (JSON 처리)
- **jsonwebtoken**, **argon2**: 인증 및 보안

### 3. 환경 변수 설정 (Configuration)
서버 실행에 필요한 설정을 합니다. `dotenvy` 라이브러리가 `.env` 파일을 로드합니다.

1. `.env.example` 파일을 복사하여 `.env` 파일을 생성합니다.
   ```bash
   cp .env.example .env
   ```
2. `.env` 파일을 열어 본인의 환경에 맞게 수정합니다.
   ```ini
   DATABASE_URL=postgres://postgres:password@localhost:5432/study_snake_xenon_board
   JWT_SECRET=내_비밀키_입력
   PORT=3000
   ```

### 4. 데이터베이스 설정 (Database Setup)
PostgreSQL이 설치되어 있어야 합니다.

1. **데이터베이스 생성**
   ```bash
   # psql 접속 후 실행 또는 GUI 툴 사용
   CREATE DATABASE study_snake_xenon_board;
   ```
2. **테이블 생성**
   `schema.sql` 파일을 실행하여 테이블을 생성합니다.
   ```bash
   # psql을 사용하는 경우
   psql -U postgres -d study_snake_xenon_board -f schema.sql
   ```

### 5. 서버 실행 (Run Server)
프로젝트를 컴파일하고 실행합니다.
```bash
cargo run
```
- **최초 실행 시**: 모든 의존성 패키지를 다운로드하고 컴파일하느라 시간이 다소 소요됩니다.
- **실행 완료**: 터미널에 `🚀 Server running at http://127.0.0.1:3000` 메시지가 뜨면 성공입니다.

---

## 🛠 API 사용법 (Endpoints)

### User App
| Method | URL | 설명 | Body (JSON) |
|--------|-----|------|-------------|
| POST | `/api/users/register` | 회원가입 | `{"username": "...", "password": "...", "email": "..."}` |
| POST | `/api/users/login` | 로그인 | `{"username": "...", "password": "..."}` |

### Board App
| Method | URL | 설명 | Body (JSON) |
|--------|-----|------|-------------|
| GET | `/api/boards` | 게시글 목록 | - |
| POST | `/api/boards` | 게시글 작성 | `{"title": "...", "content": "...", "author_id": "..."}` |
| GET | `/api/boards/:id` | 게시글 상세 | - |

---

## 📚 개발 팁 (Development Tips)

- **코드 체크**: 컴파일 에러나 경고를 빠르게 확인하려면 `cargo check`를 사용하세요.
- **자동 포맷팅**: 코드를 깔끔하게 정리하려면 `cargo fmt`를 실행하세요.
- **새로운 앱 추가**:
    1. `src/apps/` 아래에 새 폴더(예: `product`)를 만듭니다.
    2. `mod.rs`, `models.rs`, `handlers.rs`를 생성합니다.
    3. `src/apps/mod.rs`에 `pub mod product;`를 추가합니다.
    4. `src/main.rs` 라우터에 등록합니다.