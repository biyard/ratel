use crate::*;

#[post("/api/spaces/{space_pk}/discussions", role: SpaceUserRole, user : User)]
pub async fn create_discussion(space_pk: SpacePartition) -> Result<DiscussionResponse> {
    SpacePost::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let post = SpacePost::new(
        space_pk,
        String::new(),
        String::new(),
        String::new(),
        &user,
        None,
        None,
    );
    post.create(cli).await?;

    Ok(post.into())
}
