use crate::models::*;
use crate::types::*;
use crate::*;
use ratel_auth::User;

#[post("/api/posts/:post_pk/delete", user: User)]
pub async fn delete_post_handler(post_pk: FeedPartition, force: Option<bool>) -> Result<Post> {
    let conf = crate::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_pk.into();

    if !Post::has_permission(
        cli,
        &post_pk,
        Some(&user.pk),
        TeamGroupPermission::PostDelete,
    )
    .await?
    .1
    {
        return Err(Error::Unauthorized("No permission".into()));
    }

    let dependancies: Vec<Partition> = vec![];

    let force = force.unwrap_or(false);

    if force {
        tracing::warn!("Force delete is not implemented yet");
    } else if !dependancies.is_empty() {
        return Err(Error::BadRequest("Has dependencies".into()));
    }

    let post = Post::delete(cli, post_pk, Some(EntityType::Post)).await?;

    if post.status == PostStatus::Published {
        crate::services::delete_post_vector_async(conf.qdrant(), &post.pk);
    }

    Ok(post)
}
