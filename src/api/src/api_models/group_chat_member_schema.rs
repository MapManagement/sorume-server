use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PostGroupChatMember {
    pub profile_id: i32,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetGroupChatMember {
    pub profile_id: i32,
}
