use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::feed::*;
use crate::types::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema, aide::OperationIo,
)]
pub struct SpacePostCommentResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub updated_at: i64,
    pub created_at: i64,

    pub content: String,

    pub likes: u64,
    pub replies: u64,

    pub parent_comment_sk: Option<EntityType>,

    pub author_pk: Partition,
    pub author_display_name: String,
    pub author_username: String,
    pub author_profile_url: String,

    pub liked: bool,
}

impl From<(SpacePostComment, bool)> for SpacePostCommentResponse {
    fn from((comment, liked): (SpacePostComment, bool)) -> Self {
        Self {
            pk: comment.pk,
            sk: comment.sk,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            content: comment.content,
            likes: comment.likes,
            replies: comment.replies,
            parent_comment_sk: comment.parent_comment_sk,
            author_pk: comment.author_pk,
            author_display_name: comment.author_display_name,
            author_username: comment.author_username,
            author_profile_url: comment.author_profile_url,
            liked,
        }
    }
}

impl From<SpacePostComment> for SpacePostCommentResponse {
    fn from(comment: SpacePostComment) -> Self {
        Self {
            pk: comment.pk,
            sk: comment.sk,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            content: comment.content,
            likes: comment.likes,
            replies: comment.replies,
            parent_comment_sk: comment.parent_comment_sk,
            author_pk: comment.author_pk,
            author_display_name: comment.author_display_name,
            author_username: comment.author_username,
            author_profile_url: comment.author_profile_url,
            liked: false,
        }
    }
}
