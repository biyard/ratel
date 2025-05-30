use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/feeds", table = feed_comments)]
pub struct FeedComment {
    #[api_model(summary, primary_key, read_action = [find_by_id])]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub commenter_url: String,
    #[api_model(summary)]
    pub commenter_name: String,

    #[api_model(summary, many_to_one = feeds)]
    pub feed_id: i64,
    #[api_model(summary, nullable)]
    pub parent_id: Option<i64>,
    #[api_model(action = create, summary)]
    pub comment: String, //html format
                         // #[api_model(one_to_many = feed_comments, foreign_key = comment_id)]
                         // pub replies: Vec<FeedComment>,
}
