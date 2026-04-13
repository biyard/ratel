use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::auth::User;

#[mcp_tool(name = "delete_post", description = "Delete a post by ID.")]
#[delete("/api/posts/:post_id", user: User)]
pub async fn delete_post_handler(post_id: FeedPartition, force: Option<bool>) -> Result<Post> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();

    if !Post::has_permission(
        cli,
        &post_pk,
        Some(&user.pk),
        TeamGroupPermission::PostDelete,
    )
    .await?
    .1
    {
        return Err(PostError::NotAccessible.into());
    }

    let dependancies: Vec<Partition> = vec![];

    let force = force.unwrap_or(false);

    if force {
        tracing::warn!("Force delete is not implemented yet");
    } else if !dependancies.is_empty() {
        return Err(PostError::HasDependencies.into());
    }

    let post = Post::delete(cli, post_pk, Some(EntityType::Post)).await?;

    // Qdrant vector deletion is handled by DynamoStream → PostVectorDelete event

    Ok(post)
}
