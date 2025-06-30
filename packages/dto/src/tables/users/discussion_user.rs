use bdk::prelude::*;

#[derive(validator::Validate)]
#[api_model(table = users)]
pub struct DiscussionUser {
    #[api_model(primary_key)]
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub nickname: String,
    pub principal: String,
    pub profile_url: String,

    pub username: String,
}
