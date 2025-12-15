use crate::models::feed::*;
use crate::types::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema, aide::OperationIo,
)]
pub struct PostCommentResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub updated_at: i64,

    pub content: String,

    pub likes: u64,
    pub reports: i64,
    pub replies: u64,

    pub parent_comment_sk: Option<EntityType>,

    pub author_pk: Partition,
    pub author_display_name: String,
    pub author_username: String,
    pub author_profile_url: String,

    pub liked: bool,
    pub is_report: bool,
}

impl From<(PostComment, bool, bool)> for PostCommentResponse {
    fn from((comment, liked, is_report): (PostComment, bool, bool)) -> Self {
        Self {
            pk: comment.pk,
            sk: comment.sk,
            updated_at: comment.updated_at,
            content: comment.content,
            likes: comment.likes,
            reports: comment.reports,
            replies: comment.replies,
            parent_comment_sk: comment.parent_comment_sk,
            author_pk: comment.author_pk,
            author_display_name: comment.author_display_name,
            author_username: comment.author_username,
            author_profile_url: comment.author_profile_url,
            liked,
            is_report,
        }
    }
}

impl From<PostComment> for PostCommentResponse {
    fn from(comment: PostComment) -> Self {
        Self {
            pk: comment.pk,
            sk: comment.sk,
            updated_at: comment.updated_at,
            content: comment.content,
            likes: comment.likes,
            reports: comment.reports,
            replies: comment.replies,
            parent_comment_sk: comment.parent_comment_sk,
            author_pk: comment.author_pk,
            author_display_name: comment.author_display_name,
            author_username: comment.author_username,
            author_profile_url: comment.author_profile_url,
            liked: false,
            is_report: false,
        }
    }
}
