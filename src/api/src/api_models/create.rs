use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewGroupChatSchema {
    pub member_ids: Vec<i32>,
    pub group_picture: Option<String>,
}

#[derive(Deserialize)]
pub struct NewGroupChatMember {
    pub profile_id: i32
}
