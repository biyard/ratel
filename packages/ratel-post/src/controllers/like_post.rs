use crate::models::*;
use crate::*;
use ratel_auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LikePostResponse {
    pub like: bool,
}

#[post("/api/posts/:post_pk/like", user: User)]
pub async fn like_post_handler(post_pk: FeedPartition, like: bool) -> Result<LikePostResponse> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_pk.into();

    if like {
        Post::like(cli, post_pk, user.pk).await?;
        Ok(LikePostResponse { like: true })
    } else {
        Post::unlike(cli, post_pk, user.pk).await?;
        Ok(LikePostResponse { like: false })
    }
}
