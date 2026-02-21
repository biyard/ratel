use super::*;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostMetadata {
    Post(Post),
    PostComment(PostComment),
    PostArtwork(PostArtwork),
    PostRepost(PostRepost),
}

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum PostLikeMetadata {
    PostLike(PostLike),
    PostCommentLike(PostCommentLike),
}
