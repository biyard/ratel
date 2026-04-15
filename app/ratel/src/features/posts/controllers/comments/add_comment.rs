use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::auth::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddPostCommentRequest {
    pub content: String,
    #[serde(default)]
    pub images: Vec<String>,
}

#[post("/api/posts/:post_id/comments", user: User)]
pub async fn add_comment_handler(
    post_id: FeedPartition,
    req: AddPostCommentRequest,
) -> Result<PostComment> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.clone().into();

    let post = Post::get(cli, &post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error::NotFound("Post not found".into()))?;

    match &post.user_pk {
        team_pk if matches!(team_pk, &Partition::Team(_)) => {
            Team::has_permission(cli, team_pk, &user.pk, TeamGroupPermission::PostRead).await?;
        }
        _ => {}
    }

    let comment = Post::comment(cli, post.pk.clone(), req.content, req.images, user.clone()).await?;

    // Send mention notifications
    {
        let cta_url = format!(
            "{}/posts/{}",
            crate::common::config::site_base_url(),
            post_id
        );
        crate::common::utils::mention::create_mention_notifications(
            cli,
            &comment.content,
            &user.pk,
            &user.display_name,
            &cta_url,
        )
        .await;
    }

    Ok(comment)
}
