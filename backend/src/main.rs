use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // 라우터 정의
    let app = Router::new().route("/", get(hello_world));

    // 바인딩할 주소
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();

    // TcpListener 생성
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server running at http://{}", listener.local_addr().unwrap());

    // axum 0.8 스타일 서버 실행
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, Axum!"
}
