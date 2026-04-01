use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::IntoResponse,
};
use std::sync::Arc;

use crate::state::AppState;
use super::session::handle_socket;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}