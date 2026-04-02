use crate::state::AppState;
use axum::extract::ws::WebSocket;
use std::sync::Arc;
use axum::extract::ws::Message;
use futures::StreamExt;
use crate::chat::message::ChatMessage;

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();

    // 🧑 Step 1: get username (first message)
    let username = match socket.next().await {
        Some(Ok(Message::Text(name))) => name,
        _ => return,
    };

    // 📢 announce join
    let _ = state.tx.send(
        serde_json::to_string(&ChatMessage::Join {
            user: username.clone(),
        }).unwrap()
    );

    println!("{} joined", username);

    loop {
        tokio::select! {
            // 📩 message from this user
            Some(Ok(msg)) = socket.next() => {
                if let Message::Text(text) = msg {
                    let chat_msg = ChatMessage::Chat {
                        user: username.clone(),
                        content: text,
                    };

                    let _ = state.tx.send(
                        serde_json::to_string(&chat_msg).unwrap()
                    );
                }
            }

            // 📡 message from others
            Ok(msg) = rx.recv() => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }

            else => break,
        }
    }

    // 🚪 announce leave
    let _ = state.tx.send(
        serde_json::to_string(&ChatMessage::Leave {
            user: username.clone(),
        }).unwrap()
    );

    println!("{} left", username);
}