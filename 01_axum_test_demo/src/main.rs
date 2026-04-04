use axum::{
    body::Body,
    http::{Request, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize)]
pub struct CalcInput {
    a: i32,
    b: i32,
}

#[derive(Serialize)]
pub struct CalcOutput {
    result: i32,
}

async fn compute(Json(payload): Json<CalcInput>) -> Json<CalcOutput> {
    let sum = payload.a + payload.b;
    Json(CalcOutput { result: sum })
}


// 1. 라우터 생성 함수 (테스트에서 재사용)

pub fn app() -> Router {
    Router::new()
    .route("/user", get(|| async { Json(User { id: 1, username: "alice".into() }) }))
    .route("/echo", post(|Json(payload): Json<serde_json::Value>| async move { Json(payload) }))
    .route("/calc", post(compute))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

// 테스트 모듈 (파일 하단에 위치)
#[cfg(test)]
mod tests {
    use super::*; // 상위의 app, User 등을 가져옴
    use http_body_util::BodyExt;
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_user() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(body, json!({ "id": 1, "username": "alice" }));
    }
    
    #[tokio::test]
    async fn test_calc_success() {
        let app = app();

        // 1. POST 요청 생성 (JSON 데이터 포함)
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/calc")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({ "a": 10, "b": 20 }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // 2. 상태 코드 확인
        assert_eq!(response.status(), StatusCode::OK);

        // 3. 결과값 확인 (10 + 20 = 30)
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(body["result"], 30);
    }

    #[tokio::test]
    async fn test_calc_invalid_input() {
        let app = app();

        // 잘못된 형식의 데이터(문자열)를 보냈을 때 400 Bad Request가 뜨는지 확인
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/calc")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({ "a": "wrong", "b": 20 }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY); // 422 또는 400
    }    
}
