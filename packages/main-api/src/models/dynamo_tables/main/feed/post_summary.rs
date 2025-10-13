use super::{post_comment_like::PostCommentLike, *};
use bdk::prelude::*;

/*
FIXME: Remove PostComment / PostLike from PostMetadata

The reason PostComment and PostLike are included in the metadata is to make it easier to construct PostDetailResponse.

However, since the number of PostComment and PostLike can grow indefinitely,
including them in PostMetadata may cause the size to exceed 1MB, preventing normal creation of a Post.
Therefore, the PKs for PostComment and PostLike should be managed separately from the Post, and queries should also be performed separately.

However, if PostComment and PostLike are not included in PostDetailResponse, an error occurs with untagged serialization.
To prevent this, they are temporarily included in PostMetadata.

*/
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostMetadata {
    Post(Post),
    PostComment(PostComment),
    PostArtwork(PostArtwork),
    PostRepost(PostRepost),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostLikeMetadata {
    PostLike(PostLike),
    PostCommentLike(PostCommentLike),
}
