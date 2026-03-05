use crate::models::*;
use crate::types::*;
use crate::*;
use ratel_auth::User;

#[post("/api/posts/:post_id/comments", user: User)]
pub async fn add_comment_handler(post_id: FeedPartition, content: String) -> Result<PostComment> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();

    let post = Post::get(cli, &post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error::NotFound("Post not found".into()))?;

    match &post.user_pk {
        team_pk if matches!(team_pk, &Partition::Team(_)) => {
            Team::has_permission(cli, team_pk, &user.pk, TeamGroupPermission::PostRead).await?;
        }
        _ => {}
    }

    let comment = Post::comment(cli, post.pk.clone(), content, user).await?;

    Ok(comment)
}
