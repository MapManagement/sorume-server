use serde::{Deserialize, Serialize};
use serde_with::chrono::NaiveDateTime;
use utoipa::ToSchema;

// TODO: group picture
#[derive(Deserialize, ToSchema)]
pub struct PostGroupChat {
    pub member_ids: Vec<i32>,
    pub group_picture: Option<String>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct GetGroupChat {
    pub creation_date: NaiveDateTime,
    pub group_picture: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct PatchGroupChat {
    pub group_picture: String,
}
