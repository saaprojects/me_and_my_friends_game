use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong,
}
