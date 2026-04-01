use axum::{routing::get, Router, response::Html};
use crate::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new().route("/", get(home))
}

async fn home() -> Html<&'static str> {
    Html("<h1>Chat server running</h1>")
}