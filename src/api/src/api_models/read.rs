use serde::Deserialize;

#[derive(Deserialize)]
pub struct PrivateChatSchema {
    pub sender_id: i32,
    pub recipient_id: i32,
}
