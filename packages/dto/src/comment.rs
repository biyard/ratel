#![allow(unused_variables, unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;
use validator::ValidationError;

#[api_model(base = "/v1/topics/:topic-id/comments", table = comments, iter_type = QueryResponse)]
pub struct Comment {
    #[api_model(summary, primary_key, read_action = [get_comment, find_by_id])]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: u64,

    #[api_model(summary)]
    pub profile_url: String,

    // TODO: custom function from topics and users joined table
    // #[api_model(summary)]
    // pub choice: Option<String>,
    #[api_model(summary)]
    pub nickname: String,
    #[api_model(summary, action = comment, related = String)]
    pub content: String,

    #[api_model(many_to_one = topics)]
    pub topic_id: i64,

    #[api_model(summary, one_to_many = user_comments, aggregator = count)]
    pub likes: u64,
    #[api_model(many_to_many = user_comments, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = comment_id, aggregator = exist, exist)]
    pub is_liked: bool,
}
