use axum::Router; // Note: check your import, usually 'axum'
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // FIX: Use fallback_service instead of nest_service for the root "/"
    let app = Router::new().fallback_service(ServeDir::new("./static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");
    
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.expect("Server failed");
}

