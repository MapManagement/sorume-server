use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PostPrivateMessage {
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetPrivateMessage {
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct PatchPrivateMessage {
    pub content: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct DeletePostPrivateChat {
    pub sender_id: i32,
    pub recipient_id: i32,
}
