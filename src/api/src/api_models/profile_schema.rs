use serde::{Deserialize, Serialize};
use serde_with::chrono::NaiveDateTime;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PostProfile {
    pub username: String,
    pub displayname: Option<String>,
    pub password: String,
    pub email_address: String,
}

// TODO: profile picture
#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetProfile {
    pub username: String,
    pub displayname: String,
    pub email_address: String,
    pub join_datetime: NaiveDateTime,
}

#[derive(Deserialize, ToSchema)]
pub struct PatchProfile {
    pub username: Option<String>,
    pub displayname: Option<String>,
    pub password: Option<String>,
    pub email_address: Option<String>,
    pub profile_picture: Option<String>,
}
