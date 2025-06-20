use bdk::prelude::*;
use validator::Validate;

use crate::*;

#[derive(Validate)]
#[api_model(base = "/", table = feeds)]
pub struct Comment {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(summary, type = INTEGER)]
    pub feed_type: FeedType,

    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, nullable)]
    pub parent_id: Option<i64>,

    #[api_model(summary, nullable)]
    pub quote_feed_id: Option<i64>,

    #[api_model(summary)]
    pub html_contents: String,

    #[api_model(summary, many_to_many = feed_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = feed_id, aggregator = count)]
    pub num_of_likes: i64,

    #[api_model(summary, many_to_many = feed_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = feed_id, aggregator = exist)]
    pub is_liked: bool,

    #[api_model(summary, one_to_many = feeds, foreign_key = parent_id, aggregator=count)]
    pub num_of_replies: i64,

    #[api_model(summary, one_to_many = feeds, foreign_key = parent_id, nested)]
    #[serde(default)]
    pub replies: Vec<Reply>,

    #[api_model(one_to_many = users, reference_key = user_id, foreign_key = id, summary)]
    #[serde(default)]
    pub author: Vec<FeedAuthor>,
}

#[derive(Validate)]
#[api_model(base = "/", table = feeds)]
pub struct Reply {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(summary, type = INTEGER)]
    pub feed_type: FeedType,

    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, nullable)]
    pub parent_id: Option<i64>,

    #[api_model(summary)]
    pub html_contents: String,

    #[api_model(one_to_many = users, reference_key = user_id, foreign_key = id, summary)]
    #[serde(default)]
    pub author: Vec<FeedAuthor>,
}
