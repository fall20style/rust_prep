use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
}

#[derive(Serialize)]
struct CommandResponse {
    output: String,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_bash_cli=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build our application with a route
    let app = Router::new()
        .route("/api/execute", post(execute_command))
        .fallback_service(ServeDir::new("static"));

    // Run it
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn execute_command(Json(payload): Json<CommandRequest>) -> impl IntoResponse {
    info!("Executing command: {}", payload.command);

    // Using tokio::process::Command for non-blocking execution
    let output = Command::new("bash")
        .arg("-c")
        .arg(&payload.command)
        .output()
        .await;

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            
            let response = CommandResponse {
                output: stdout,
                error: if stderr.is_empty() { None } else { Some(stderr) },
            };
            Json(response)
        }
        Err(e) => {
            let response = CommandResponse {
                output: "".to_string(),
                error: Some(format!("Failed to execute command: {}", e)),
            };
            Json(response)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt; // for `oneshot`, `ready`, and `call`
    use serde_json::{json, Value};

    #[tokio::test]
    async fn test_execute_ls() {
        let app = Router::new().route("/api/execute", post(execute_command));

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/execute")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!({ "command": "ls" })).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        
        assert!(body.get("output").unwrap().as_str().is_some());
    }

    #[tokio::test]
    async fn test_execute_error() {
        let app = Router::new().route("/api/execute", post(execute_command));

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/api/execute")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!({ "command": "nonexistentcommand" })).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        
        assert!(body.get("error").unwrap().as_str().is_some());
    }
}
