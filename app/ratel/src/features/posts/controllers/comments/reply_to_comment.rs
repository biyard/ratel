use crate::features::posts::models::*;
use crate::features::posts::*;
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ReplyToPostCommentRequest {
    pub content: String,
    #[serde(default)]
    pub images: Vec<String>,
}

#[post("/api/posts/:post_id/comments/:comment_id/reply", user: User)]
pub async fn reply_to_comment_handler(
    post_id: FeedPartition,
    comment_id: PostCommentEntityType,
    req: ReplyToPostCommentRequest,
) -> Result<PostComment> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();
    let comment_sk: EntityType = comment_id.into();

    tracing::debug!("Handling reply to comment: {:?}", comment_sk);

    let comment = PostComment::reply(cli, post_pk, comment_sk, req.content, req.images, user).await?;

    Ok(comment)
}
