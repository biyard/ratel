use bdk::prelude::*;
use validator::Validate;

use crate::Author;

use super::FeedType;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/comments", table = feeds)]
pub struct SpaceComment {
    #[api_model(primary_key)]
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub html_contents: String,

    #[api_model(type = INTEGER)]
    pub feed_type: FeedType,

    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(many_to_one = feeds)]
    pub parent_id: i64,

    #[api_model(one_to_many = users, reference_key = user_id, foreign_key = id)]
    pub author: Vec<Author>,
}
