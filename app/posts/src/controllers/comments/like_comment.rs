use crate::models::*;
use crate::*;
use ratel_auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LikeCommentResponse {
    pub liked: bool,
}

#[post("/api/posts/:post_pk/comments/:comment_sk/like", user: User)]
pub async fn like_comment_handler(
    post_pk: FeedPartition,
    comment_sk: EntityType,
    like: bool,
) -> Result<LikeCommentResponse> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_pk.into();

    tracing::debug!("Handling like comment request: like = {}", like);

    if like {
        Post::like_comment(cli, post_pk, comment_sk, user.pk).await?;
    } else {
        Post::unlike_comment(cli, post_pk, comment_sk, user.pk).await?;
    }

    Ok(LikeCommentResponse { liked: like })
}
