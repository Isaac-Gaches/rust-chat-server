use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap();

    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health));

    println!("Server running on {}", addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}