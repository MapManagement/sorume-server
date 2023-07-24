use serde::{Deserialize, Serialize};
use serde_with::chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct PostProfile {
    pub username: String,
    pub password: String,
    pub email_address: String,
}

// TODO: profile picture
#[derive(Deserialize, Serialize)]
pub struct GetProfile {
    pub username: String,
    pub email_address: String,
    pub join_datetime: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct PatchProfile {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email_address: Option<String>,
    pub profile_picture: Option<String>,
}
