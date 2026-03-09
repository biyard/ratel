use crate::features::posts::models::*;
use crate::features::posts::*;
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LikeCommentResponse {
    pub liked: bool,
}

#[post("/api/posts/:post_id/comments/:comment_id/like", user: User)]
pub async fn like_comment_handler(
    post_id: FeedPartition,
    comment_id: PostCommentEntityType,
    liked: bool,
) -> Result<LikeCommentResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();
    let comment_sk: EntityType = comment_id.into();

    tracing::debug!("Handling like comment request: like = {}", liked);

    if liked {
        Post::like_comment(cli, post_pk, comment_sk, user.pk).await?;
    } else {
        Post::unlike_comment(cli, post_pk, comment_sk, user.pk).await?;
    }

    Ok(LikeCommentResponse { liked })
}
