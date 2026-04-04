use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct CalcRequest {
    a: f64,
    b: f64,
}

#[derive(Serialize, Deserialize)]
struct CalcResponse {
    result: f64,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn add(Json(payload): Json<CalcRequest>) -> impl IntoResponse {
    (StatusCode::OK, Json(CalcResponse { result: payload.a + payload.b }))
}

async fn subtract(Json(payload): Json<CalcRequest>) -> impl IntoResponse {
    (StatusCode::OK, Json(CalcResponse { result: payload.a - payload.b }))
}

async fn multiply(Json(payload): Json<CalcRequest>) -> impl IntoResponse {
    (StatusCode::OK, Json(CalcResponse { result: payload.a * payload.b }))
}

async fn divide(Json(payload): Json<CalcRequest>) -> impl IntoResponse {
    if payload.b == 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::to_value(ErrorResponse {
                error: "Division by zero".to_string(),
            }).unwrap()),
        ).into_response();
    }
    (StatusCode::OK, Json(CalcResponse { result: payload.a / payload.b })).into_response()
}

use tower_http::cors::{Any, CorsLayer};

pub fn app() -> Router {
    Router::new()
        .route("/add", post(add))
        .route("/subtract", post(subtract))
        .route("/multiply", post(multiply))
        .route("/divide", post(divide))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`
    use serde_json::json;

    #[tokio::test]
    async fn test_add() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/add")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"a": 10.0, "b": 5.0}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let res: CalcResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(res.result, 15.0);
    }

    #[tokio::test]
    async fn test_subtract() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/subtract")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"a": 10.0, "b": 5.0}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let res: CalcResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(res.result, 5.0);
    }

    #[tokio::test]
    async fn test_multiply() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/multiply")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"a": 10.0, "b": 5.0}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let res: CalcResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(res.result, 50.0);
    }

    #[tokio::test]
    async fn test_divide() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/divide")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"a": 10.0, "b": 5.0}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let res: CalcResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(res.result, 2.0);
    }

    #[tokio::test]
    async fn test_divide_by_zero() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/divide")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"a": 10.0, "b": 0.0}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
