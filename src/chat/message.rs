use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ChatMessage {
    Join { user: String },
    Leave { user: String },
    Chat { user: String, content: String },
}