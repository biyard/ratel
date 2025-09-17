use bdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::*;

#[api_model(table = feeds)]
pub struct Post {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model( auto = [insert])]
    pub created_at: i64,
    #[api_model( auto = [insert, update])]
    pub updated_at: i64,

    #[api_model( type = INTEGER, queryable)]
    pub feed_type: FeedType,

    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(many_to_one = industries)]
    pub industry_id: i64,

    // parent Post ID
    #[api_model(nullable, indexed)]
    pub parent_id: Option<i64>,

    #[api_model(nullable, indexed)]
    pub quote_feed_id: Option<i64>,

    // Post
    #[api_model(nullable)]
    pub title: Option<String>,

    pub html_contents: String,

    #[api_model(nullable)]
    pub url: Option<String>,
    #[api_model(type = INTEGER)]
    pub url_type: UrlType,

    #[api_model(one_to_many = spaces, foreign_key = feed_id, nested)]
    pub space: Vec<Space>,

    #[api_model(many_to_many = feed_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = feed_id, aggregator = count)]
    #[serde(default)]
    pub likes: i64,

    #[api_model(many_to_many = feed_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = feed_id, aggregator = exist)]
    #[serde(default)]
    pub is_liked: bool,

    #[api_model(one_to_many = feeds, foreign_key = parent_id, aggregator=count)]
    #[serde(default)]
    pub comments: i64,

    #[api_model(one_to_many = feeds, foreign_key = parent_id, nested)]
    #[serde(default)]
    pub comment_list: Vec<Comment>,

    #[api_model(type = JSONB)]
    #[serde(default)]
    pub files: Vec<File>,

    pub rewards: i64,

    #[api_model(many_to_many = feed_shares, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = feed_id, aggregator = count)]
    pub shares: i64,

    #[api_model(type = INTEGER, queryable)]
    pub status: FeedStatus,

    #[api_model(one_to_many = users, reference_key = user_id, foreign_key = id)]
    #[serde(default)]
    pub author: Vec<FeedAuthor>,

    #[api_model(one_to_many = industries, reference_key = industry_id, foreign_key = id, summary)]
    pub industry: Vec<Industry>,

    #[api_model( many_to_many = feed_bookmark_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = feed_id, aggregator = exist)]
    pub is_bookmarked: bool,

    #[api_model(one_to_many = onboards, foreign_key = meta_id,  aggregator = exist)]
    pub onboard: bool,

    #[api_model(version = v0.4, type = JSONB)]
    #[serde(default)]
    pub artwork_metadata: ArtworkMetadata,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ArtworkMetadata {
    // image : post.url
    // name: post.title,
    // description: post.html_contents
    #[serde(default)]
    pub traits: Vec<ArtworkTrait>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ArtworkTrait {
    pub trait_type: String,
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_type: Option<String>,
}
