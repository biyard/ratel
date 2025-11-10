use crate::File;
use crate::features::spaces::boards::dto::space_post_comment_response::SpacePostCommentResponse;
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::types::Attribute;
use crate::{
    features::spaces::panels::SpacePanel,
    types::{EntityType, Partition},
};
use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpacePostResponse {
    pub pk: Partition,

    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub html_contents: String,
    pub category_name: String,
    pub number_of_comments: i64,

    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub urls: Vec<String>,
    pub files: Vec<File>,
    pub comments: Vec<SpacePostCommentResponse>,
}

impl From<SpacePost> for SpacePostResponse {
    fn from(post: SpacePost) -> Self {
        Self {
            pk: match post.sk {
                EntityType::SpacePost(v) => Partition::SpacePost(v.to_string()),
                _ => Partition::SpacePost("".to_string()),
            },
            created_at: post.created_at,
            updated_at: post.updated_at,
            title: post.title,
            html_contents: post.html_contents,
            category_name: post.category_name,
            number_of_comments: post.comments,

            user_pk: post.user_pk,
            author_display_name: post.author_display_name,
            author_profile_url: post.author_profile_url,
            author_username: post.author_username,

            urls: post.urls,
            files: post.files.unwrap_or_default(),
            comments: vec![],
        }
    }
}
