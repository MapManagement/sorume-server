use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PostPrivateMessage {
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetPrivateMessage {
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchPrivateMessage {
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeletePostPrivateChat {
    pub sender_id: i32,
    pub recipient_id: i32,
}
