mod app;
mod state;
mod web_socket;
mod routes;
mod chat;

#[tokio::main]
async fn main() {
    app::run().await;
}