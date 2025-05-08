use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/feeds", table = feeds, action = [repost(industry_id = Option<i64>)], action_by_id = [delete])]
pub struct Feed {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [write_post, comment, review_doc, repost], action_by_id = [update])]
    pub html_contents: String,

    #[api_model(summary, type = INTEGER)]
    pub feed_type: FeedType,

    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, many_to_one = industries, action = [write_post])]
    pub industry_id: i64,

    // parent feed ID
    #[api_model(summary, nullable, indexed, action = [review_doc, comment, repost])]
    pub parent_id: Option<i64>,

    // Post
    #[api_model(summary, nullable, action = [write_post])]
    pub title: Option<String>,

    // DocsReview
    #[api_model(summary, nullable, indexed, action = [review_doc])]
    pub part_id: Option<i64>,
    // Reply

    // Repost
    #[api_model(summary, nullable, indexed, action = [repost])]
    pub quote_feed_id: Option<i64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum FeedType {
    #[default]
    Post = 1,
    Reply = 2,
    Repost = 3,
    DocReview = 4,
}
