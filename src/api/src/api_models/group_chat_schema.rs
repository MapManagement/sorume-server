use serde::{Deserialize, Serialize};
use serde_with::chrono::NaiveDateTime;

// TODO: group picture
#[derive(Deserialize)]
pub struct PostGroupChat {
    pub member_ids: Vec<i32>,
    pub group_picture: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GetGroupChat {
    pub creation_date: NaiveDateTime,
    pub group_picture: Option<String>,
}

#[derive(Deserialize)]
pub struct PatchGroupChat {
    pub group_picture: String,
}
