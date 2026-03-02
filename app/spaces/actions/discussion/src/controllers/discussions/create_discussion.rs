use crate::*;

#[post("/api/spaces/{space_id}/discussions", role: SpaceUserRole, user: ratel_auth::User)]
pub async fn create_discussion(space_id: SpacePartition) -> Result<SpacePost> {
    SpacePost::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let post = SpacePost::new(
        space_id,
        String::new(),
        String::new(),
        String::new(),
        &user,
        None,
        None,
    );
    post.create(cli).await?;

    Ok(post)
}
