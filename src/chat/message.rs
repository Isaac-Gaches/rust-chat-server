use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub user: String,
    pub content: String,
}