use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Html,
    routing::get,
    Router,
};
use bollard::Docker;
use bollard::exec::{CreateExecOptions, StartExecResults};
use futures_util::{StreamExt, SinkExt};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_handler))
        .route("/xterm.js", get(|| async { 
            ([(axum::http::header::CONTENT_TYPE, "application/javascript")], include_str!("xterm.js")) 
        }))
        .route("/xterm.css", get(|| async { 
            ([(axum::http::header::CONTENT_TYPE, "text/css")], include_str!("xterm.css")) 
        }));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 서버 오픈: http://localhost:3000");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn ws_handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(|socket| handle_socket(socket))
}

async fn handle_socket(socket: WebSocket) {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let container_name = "my_terminal"; 

    let config = CreateExecOptions {
        attach_stdin: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        tty: Some(true),
        env: Some(vec!["TERM=xterm"]),
        cmd: Some(vec!["bash"]),
        ..Default::default()
    };

    let exec_id = match docker.create_exec(container_name, config).await {
        Ok(res) => res.id,
        Err(_) => return,
    };

    if let Ok(StartExecResults::Attached { mut output, mut input }) = 
        docker.start_exec(&exec_id, None).await {
        
        let (mut ws_sender, mut ws_receiver) = socket.split();

        // Docker -> WebSocket
        let mut d_to_w = tokio::spawn(async move {
            while let Some(Ok(msg)) = output.next().await {
                let bytes = msg.into_bytes().to_vec();
                if ws_sender.send(Message::Binary(bytes)).await.is_err() { break; }
            }
        });

        // WebSocket -> Docker
        let mut w_to_d = tokio::spawn(async move {
            while let Some(Ok(msg)) = ws_receiver.next().await {
                match msg {
                    Message::Binary(bin) => { let _ = input.write_all(&bin).await; }
                    Message::Text(t) => { let _ = input.write_all(t.as_bytes()).await; }
                    _ => {}
                }
                let _ = input.flush().await;
            }
        });

        tokio::select! {
            _ = (&mut d_to_w) => w_to_d.abort(),
            _ = (&mut w_to_d) => d_to_w.abort(),
        }
    }
}

