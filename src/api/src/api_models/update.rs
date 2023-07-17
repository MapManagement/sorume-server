use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateProfileSchema {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email_address: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateGroupChatSchema {
    pub group_picture: String
}

#[derive(Deserialize)]
pub struct UpdatePrivateMessage {
    pub content: String
}
