use bdk::prelude::*;
use validator::Validate;

use crate::{File, Space};

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

    #[api_model(summary, many_to_one = users, action = [write_post, comment, review_doc, repost])]
    pub user_id: i64,

    #[api_model(summary, many_to_one = industries, action = [write_post])]
    pub industry_id: i64,

    #[api_model(version = v0.1, summary, nullable)]
    #[serde(default)]
    pub proposer_name: Option<String>,

    #[api_model(version = v0.1, summary, nullable)]
    #[serde(default)]
    pub profile_image: Option<String>,

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
    #[api_model(summary, one_to_many = feed_comments, foreign_key = feed_id, aggregator=count)]
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
