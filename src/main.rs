use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(ws_handler));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".into());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("Server running on port 3000");

    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("Client connected");

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received: {}", text);

                if socket.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => {
                println!("Client disconnected");
                break;
            }
            _ => {}
        }
    }
}
