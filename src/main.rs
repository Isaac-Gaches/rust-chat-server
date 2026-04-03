mod app;
mod state;
mod web_socket;
mod routes;
mod chat;
mod rate_limit;

#[tokio::main]
async fn main() {
    app::run().await;
}