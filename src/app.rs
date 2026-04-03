use axum::Router;
use std::sync::Arc;

use crate::{state::AppState, routes, web_socket};
use logger::*;

pub async fn run() {
    logger::init(Level::Info,1000);
    log!(Level::Info, "Server starting");

    let state = Arc::new(AppState::new());

    let app = Router::new()
        .merge(routes::routes())
        .merge(web_socket::routes())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{}", port);

    log!(Level::Info, "Address found: {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    log!(Level::Info, "Server Running");

    axum::serve(listener, app).await.unwrap();
}