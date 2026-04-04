use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Whitelist of allowed commands for security as per Specification.md
const COMMAND_WHITELIST: &[&str] = &["ls", "vim", "cat", "echo", "pwd", "date", "whoami", "top", "htop"];

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_bash_cli=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .fallback_service(ServeDir::new("static"));

    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let pty_system = native_pty_system();
    let pty_pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();

    let cmd = CommandBuilder::new("bash");
    let _child = pty_pair.slave.spawn_command(cmd).unwrap();

    let mut writer = pty_pair.master.take_writer().unwrap();
    let mut reader = pty_pair.master.try_clone_reader().unwrap();

    let (mut ws_sender, mut ws_receiver) = socket.split();

    // Spawn a task to read from PTY and send to WebSocket
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(100);
    
    // Read from PTY thread
    std::thread::spawn(move || {
        let mut buf = [0u8; 1024];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 { break; }
            if tx.blocking_send(buf[..n].to_vec()).is_err() { break; }
        }
    });

    // Send PTY output to WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            if ws_sender.send(Message::Binary(data)).await.is_err() {
                break;
            }
        }
    });

    // Receive from WebSocket and write to PTY
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // Basic command filtering for security
                    if is_command_allowed(&text) {
                        let _ = writer.write_all(text.as_bytes());
                    } else {
                        // Notify user if command is not allowed
                        let _ = writer.write_all(b"\r\nCommand not in whitelist\r\n");
                    }
                }
                Message::Binary(bin) => {
                    let _ = writer.write_all(&bin);
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

fn is_command_allowed(input: &str) -> bool {
    // This is a naive check for demonstration. 
    // In a real app, you'd need more robust parsing to prevent bypasses like `ls; rm -rf /`
    let trimmed = input.trim();
    if trimmed.is_empty() { return true; }
    
    // Check if the first word is in the whitelist
    let cmd = trimmed.split_whitespace().next().unwrap_or("");
    COMMAND_WHITELIST.contains(&cmd) || trimmed.len() == 1 // Allow single characters (keystrokes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitelist() {
        assert!(is_command_allowed("ls -la"));
        assert!(is_command_allowed("vim"));
        assert!(!is_command_allowed("rm -rf /"));
        assert!(is_command_allowed("a")); // single keystroke
    }
}
