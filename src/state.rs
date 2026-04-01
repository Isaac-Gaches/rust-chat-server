use tokio::sync::broadcast;

pub type SharedState = std::sync::Arc<AppState>;

pub struct AppState {
    pub tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }
}