use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, Response},
    routing::get,
    Router,
};
use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use futures::{sink::SinkExt, stream::StreamExt};

pub fn app() -> Router {
    Router::new()
        .route("/", get(|| async { Html(include_str!("index.html")) }))
        .route("/ws", get(handler))
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    // 1. 도커 실행 (stdbuf로 버퍼링 강제 해제)
    let mut child = Command::new("docker")
        .args(["run", "-it", "--rm", "ubuntu", "stdbuf", "-oL", "bash"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Docker 실행 실패");

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    // 2. 웹소켓을 읽기(receiver)와 쓰기(sender)로 분리 (중요!)
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Task 1: Docker Stdout -> Browser
    let stdout_task = tokio::spawn(async move {
        let mut buffer = [0u8; 1024];
        while let Ok(n) = stdout.read(&mut buffer).await {
            if n == 0 { break; }
            let _ = ws_sender.send(Message::Text(String::from_utf8_lossy(&buffer[..n]).to_string().into())).await;
        }
    });

    // Task 2: Browser -> Docker Stdin
    let stdin_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(data))) = ws_receiver.next().await {
            let _ = stdin.write_all(data.as_bytes()).await;
            let _ = stdin.flush().await;
        }
    });

    // Task 3: Docker Stderr -> Browser (에러 메시지 확인용)
    // 실제로는 별도 task 보다는 stdout과 합치는게 좋지만 일단 에러 방지용으로 비워둠
    let _stderr_task = tokio::spawn(async move {
        let mut buffer = [0u8; 1024];
        while let Ok(_) = stderr.read(&mut buffer).await {}
    });

    tokio::select! {
        _ = stdout_task => (),
        _ = stdin_task => (),
    }
    let _ = child.kill().await;
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running on http://localhost:8080");
    axum::serve(listener, app()).await.unwrap();
}

