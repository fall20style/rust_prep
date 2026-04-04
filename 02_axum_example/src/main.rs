use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

// POST 요청에서 받을 데이터 구조체
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// 응답으로 보낼 JSON 구조체
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() {
    // 1. 라우터 설정
    let app = Router::new()
        // GET 요청: 단순 문자열 응답
        .route("/", get(handler_hello))
        // POST 요청: JSON 데이터 처리
        .route("/users", post(create_user));

    // 2. 서버 실행 주소 설정 (localhost:3000)
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("서버가 http://127.0.0.1:8080 에서 실행 중입니다.");
    
    axum::serve(listener, app).await.unwrap();
}

// GET 핸들러: 단순 문자열 반환
async fn handler_hello() -> &'static str {
    "Hello, Axum GET Request!"
}

// POST 핸들러: JSON 입력받아 JSON으로 응답
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // Json()으로 감싸서 반환하면 Content-Type이 application/json으로 설정됩니다.
    Json(user)
}

