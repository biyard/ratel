use std::collections::HashSet;

use super::*;
use crate::{
    models::feed::*,
    types::EntityType,
    // types::{TeamGroupPermission, TeamGroupPermissions},
};
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct PostDetailResponse {
    pub post: Option<Post>,
    pub comments: Vec<PostCommentResponse>,
    pub artwork_metadata: Vec<PostArtworkMetadata>,
    pub repost: Option<PostRepost>,
    pub is_liked: bool,
    pub is_report: bool,
    pub permissions: i64,
}

impl From<Vec<PostMetadata>> for PostDetailResponse {
    fn from(items: Vec<PostMetadata>) -> Self {
        let mut res = Self::default();

        for item in items {
            match item {
                PostMetadata::Post(post) => res.post = Some(post),
                PostMetadata::PostComment(comment) => res.comments.push(comment.into()),
                PostMetadata::PostArtwork(artwork) => res.artwork_metadata = artwork.metadata,
                PostMetadata::PostRepost(repost) => res.repost = Some(repost),
            }
        }
        res
    }
}

// (PostMetadata, permissions, is_liked)
impl
    From<(
        Vec<PostMetadata>,
        i64,
        bool,
        bool,
        Vec<PostCommentLike>,
        HashSet<String>,
    )> for PostDetailResponse
{
    fn from(
        (items, perms, is_liked, is_report, post_comment_likes, reported_comment_ids): (
            Vec<PostMetadata>,
            i64,
            bool,
            bool,
            Vec<PostCommentLike>,
            HashSet<String>,
        ),
    ) -> Self {
        let mut res = Self::default();
        res.permissions = perms;
        res.is_liked = is_liked;
        res.is_report = is_report;

        for item in items {
            match item {
                PostMetadata::Post(post) => res.post = Some(post),
                PostMetadata::PostComment(comment) => {
                    let liked = post_comment_likes.iter().any(|like| like == comment);

                    let sk_str = comment.sk.to_string();
                    let is_comment_reported = reported_comment_ids.contains(&sk_str);

                    res.comments
                        .push((comment, liked, is_comment_reported).into());
                }
                PostMetadata::PostArtwork(artwork) => {
                    res.artwork_metadata = artwork.metadata;
                }
                PostMetadata::PostRepost(repost) => res.repost = Some(repost),
            }
        }

        res
    }
}
