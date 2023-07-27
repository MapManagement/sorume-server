use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PostGroupChatMember {
    pub profile_id: i32,
}
