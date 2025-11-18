use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 라우터 정의: GET / 에 hello_world 핸들러 연결
    let app = Router::new().route("/", get(hello_world));

    // 바인딩할 주소 (127.0.0.1:3000)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{addr}");

    // 서버 실행
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 핸들러 함수: request -> response
async fn hello_world() -> &'static str {
    "Hello, Axum!"
}
