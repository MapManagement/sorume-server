use serde::{Deserialize, Serialize};
use serde_with::chrono::NaiveDateTime;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PostGroupChatMessage {
    pub author_id: i32,
    pub content: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetGroupChatMessage {
    pub message_id: i32,
    pub author_id: i32,
    pub send_time: NaiveDateTime,
    pub content: Option<String>,
    pub chat_id: i32,
}

#[derive(Deserialize, ToSchema)]
pub struct PatchGroupChatMessage {
    pub content: String,
}
