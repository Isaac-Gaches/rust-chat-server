use crate::state::AppState;
use axum::extract::ws::WebSocket;
use std::sync::Arc;
use std::time::{Duration};
use axum::extract::ws::Message;
use futures::StreamExt;
use crate::chat::message::ChatMessage;
use crate::rate_limit::limiter::RateLimiter;
use logger::*;

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.tx.subscribe();

    let username = match socket.next().await {
        Some(Ok(Message::Text(name))) => name,
        _ => return,
    };

    let mut limiter = RateLimiter::new(5, Duration::from_secs(5));

    let _ = state.tx.send(
        serde_json::to_string(&ChatMessage::Join {
            user: username.clone(),
        }).unwrap()
    );

    log!(Level::Info, "{} joined", username.clone());

    loop {
        tokio::select! {
            msg = socket.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if limiter.allow() {
                            let _ = socket.send(Message::Text(
                                "You're sending messages too fast!".into()
                            )).await;
                            log!(Level::Warn, "{} messaging too fast", username.clone());
                            continue;
                        }

                        if text.starts_with("/logs") {
                            let level = if text.contains("-info") {
                                Level::Info
                            } else if text.contains("-warn") {
                                Level::Warn
                            } else if text.contains("-error") {
                                Level::Error
                            } else {
                                Level::Debug
                            };

                            let logs = logger::get_logs(level);

                            let response = logs
                                .iter()
                                .rev()
                                .take(20)
                                .map(|l| format!("[{:?}] {}", l.level, l.msg))
                                .collect::<Vec<_>>();

                            let chat_msg = ChatMessage::Query { response };

                            let _ = state.tx.send(
                                serde_json::to_string(&chat_msg).unwrap()
                            );
                        }
                        else {
                            let chat_msg = ChatMessage::Chat {
                                user: username.clone(),
                                content: text,
                            };

                            let _ = state.tx.send(
                                serde_json::to_string(&chat_msg).unwrap()
                            );
                        }
                    }

                    _ => break,
                }
            }
            Ok(msg) = rx.recv() => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    }

    let _ = state.tx.send(
        serde_json::to_string(&ChatMessage::Leave {
            user: username.clone(),
        }).unwrap()
    );

    log!(Level::Info, "{} left", username.clone());
}