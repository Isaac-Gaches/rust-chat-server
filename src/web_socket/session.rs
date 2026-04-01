use crate::state::AppState;
use axum::extract::ws::WebSocket;
use std::sync::Arc;
use axum::extract::ws::Message;
use futures::StreamExt;

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
            Some(Ok(msg)) = socket.next() => {
                if let Message::Text(text) = msg {
                    let _ = state.tx.send(text);
                }
            }

            Ok(msg) = rx.recv() => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    }
}