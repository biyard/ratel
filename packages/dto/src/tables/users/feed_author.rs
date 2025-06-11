use bdk::prelude::*;

use super::UserType;

#[derive(validator::Validate)]
#[api_model(table = users)]
pub struct FeedAuthor {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub nickname: String,
    pub principal: String,
    pub profile_url: String,

    #[api_model(type = INTEGER, indexed, version = v0.1)]
    pub user_type: UserType,
    pub parent_id: Option<i64>,
    pub username: String,

    // profile contents
    #[serde(default)]
    pub html_contents: String,
}
