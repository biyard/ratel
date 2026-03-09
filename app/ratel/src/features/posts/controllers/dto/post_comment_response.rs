use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PostCommentResponse {
    pub pk: FeedPartition,
    pub sk: PostCommentEntityType,

    pub updated_at: i64,

    pub content: String,

    pub likes: u64,
    pub reports: i64,
    pub replies: u64,

    pub parent_comment_sk: Option<PostCommentEntityType>,

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
            pk: comment.pk.into(),
            sk: comment.sk.into(),
            updated_at: comment.updated_at,
            content: comment.content,
            likes: comment.likes,
            reports: comment.reports,
            replies: comment.replies,
            parent_comment_sk: comment.parent_comment_sk.map(|sk| sk.into()),
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
            pk: comment.pk.into(),
            sk: comment.sk.into(),
            updated_at: comment.updated_at,
            content: comment.content,
            likes: comment.likes,
            reports: comment.reports,
            replies: comment.replies,
            parent_comment_sk: comment.parent_comment_sk.map(|sk| sk.into()),
            author_pk: comment.author_pk,
            author_display_name: comment.author_display_name,
            author_username: comment.author_username,
            author_profile_url: comment.author_profile_url,
            liked: false,
            is_report: false,
        }
    }
}
