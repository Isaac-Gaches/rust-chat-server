use axum::Router;
use std::sync::Arc;

use crate::{state::AppState, routes, web_socket};

pub async fn run() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .merge(routes::routes())
        .merge(web_socket::routes())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or("3000".into());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server running");
    axum::serve(listener, app).await.unwrap();
}