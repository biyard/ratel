use crate::features::posts::models::*;
use crate::features::posts::*;
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LikePostResponse {
    pub like: bool,
}

#[mcp_tool(name = "like_post", description = "Like or unlike a post.")]
#[post("/api/posts/:post_id/like", user: User)]
pub async fn like_post_handler(post_id: FeedPartition, like: bool) -> Result<LikePostResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();

    if like {
        Post::like(cli, post_pk.clone(), user.pk).await?;

        #[cfg(feature = "local-dev")]
        {
            // Check if the post just became popular and trigger broader fan-out
            if let Some(post) = Post::get(cli, &post_pk, Some(EntityType::Post)).await? {
                if crate::features::timeline::services::is_popular(
                    post.likes,
                    post.comments,
                    post.shares,
                ) {
                    let _ = crate::features::timeline::services::fan_out_popular_post(
                        cli,
                        &post.pk,
                        &post.user_pk,
                        post.updated_at,
                    )
                    .await
                    .map_err(|e| {
                        tracing::error!("popular post fan-out failed: {}", e);
                    });
                }
            }
        }

        Ok(LikePostResponse { like: true })
    } else {
        Post::unlike(cli, post_pk, user.pk).await?;
        Ok(LikePostResponse { like: false })
    }
}
