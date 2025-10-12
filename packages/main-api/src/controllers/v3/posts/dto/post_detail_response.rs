use super::*;
use crate::models::feed::*;
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct PostDetailResponse {
    pub post: Option<Post>,
    pub comments: Vec<PostCommentResponse>,
    pub artwork_metadata: Vec<PostArtworkMetadata>,
    pub repost: Option<PostRepost>,
    pub is_liked: bool,
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
impl From<(Vec<PostMetadata>, i64, bool, Vec<PostCommentLike>)> for PostDetailResponse {
    fn from(
        (items, perms, is_liked, post_comment_likes): (
            Vec<PostMetadata>,
            i64,
            bool,
            Vec<PostCommentLike>,
        ),
    ) -> Self {
        let mut res = Self::default();
        res.permissions = perms;
        res.is_liked = is_liked;

        for item in items {
            match item {
                PostMetadata::Post(post) => res.post = Some(post),
                PostMetadata::PostComment(comment) => {
                    let liked = post_comment_likes
                        .iter()
                        .any(|like| if like == comment { true } else { false });

                    res.comments.push((comment, liked).into());
                }
                PostMetadata::PostArtwork(artwork) => res.artwork_metadata = artwork.metadata,
                PostMetadata::PostRepost(repost) => res.repost = Some(repost),
            }
        }

        res
    }
}
