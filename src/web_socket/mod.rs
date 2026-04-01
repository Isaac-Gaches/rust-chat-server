pub mod handler;
mod session;

use axum::{routing::get, Router};
use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new().route("/ws", get(handler::ws_handler))
}