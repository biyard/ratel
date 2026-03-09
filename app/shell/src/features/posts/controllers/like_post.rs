use crate::features::posts::models::*;
use crate::features::posts::*;
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LikePostResponse {
    pub like: bool,
}

#[post("/api/posts/:post_id/like", user: User)]
pub async fn like_post_handler(post_id: FeedPartition, like: bool) -> Result<LikePostResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();

    if like {
        Post::like(cli, post_pk, user.pk).await?;
        Ok(LikePostResponse { like: true })
    } else {
        Post::unlike(cli, post_pk, user.pk).await?;
        Ok(LikePostResponse { like: false })
    }
}
