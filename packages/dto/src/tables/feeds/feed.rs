use bdk::prelude::*;
use validator::Validate;

use crate::{File, Industry, Space, User};

#[derive(Validate)]
#[api_model(base = "/v1/feeds", table = feeds, action = [], action_by_id = [delete])]
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

    #[api_model(summary, many_to_one = users, query_action = posts_by_user_id, action = [write_post, comment, review_doc, repost])]
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
    #[api_model(summary, nullable, indexed, action = [repost, write_post])]
    pub quote_feed_id: Option<i64>,

    #[api_model(summary, one_to_many = spaces, foreign_key = feed_id)]
    pub spaces: Vec<Space>,

    #[api_model(summary, many_to_many = feed_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = feed_id, aggregator = count, unique)]
    pub likes: i64,
    #[api_model(summary, one_to_many = feeds, foreign_key = parent_id, aggregator=count)]
    pub comments: i64,
    #[api_model(version = v0.1, summary, action = write_post, type = JSONB)]
    #[serde(default)]
    pub files: Vec<File>,
    #[api_model(version = v0.1, summary)]
    #[serde(default)]
    pub rewards: i64,
    #[api_model(version = v0.1, summary)]
    #[serde(default)]
    pub shares: i64,

    #[api_model(version = v0.2, summary, action = [write_post])]
    pub url: Option<String>,
    #[api_model(version = v0.2, summary, type = INTEGER, action = [write_post])]
    #[serde(default)]
    pub url_type: UrlType,

    #[api_model(one_to_many = users, reference_key = user_id, foreign_key = id, summary)]
    pub author: Vec<User>,

    #[api_model(one_to_many = industries, reference_key = industry_id, foreign_key = id, summary)]
    pub industry: Vec<Industry>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum FeedType {
    #[default]
    Post = 1,

    // Belows are kinds of comments
    Reply = 2,
    Repost = 3,
    DocReview = 4,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum UrlType {
    #[default]
    None = 0,
    Image = 1,
}
