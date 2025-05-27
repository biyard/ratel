use bdk::prelude::*;

use super::UserType;

#[derive(validator::Validate)]
#[api_model(table = users)]
pub struct Author {
    #[api_model(primary_key)]
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub nickname: String,
    pub principal: String,
    pub email: String,
    pub profile_url: String,

    #[api_model(type = INTEGER)]
    pub user_type: UserType,
    pub username: String,

    #[api_model(version = v0.2)]
    #[serde(default)]
    pub html_contents: String,
}
