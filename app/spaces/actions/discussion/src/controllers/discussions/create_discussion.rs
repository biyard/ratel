use crate::*;

#[post("/api/spaces/{space_id}/discussions", role: SpaceUserRole, user: ratel_auth::User)]
pub async fn create_discussion(space_id: SpacePartition) -> Result<SpacePost> {
    SpacePost::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let post = SpacePost::new(
        space_id.clone(),
        String::new(),
        String::new(),
        String::new(),
        &user,
        None,
        None,
    );

    let mut items = vec![post.create_transact_write_item()];
    items.extend(post.dashboard_write_items());
    cli.transact_write_items()
        .set_transact_items(Some(items))
        .send()
        .await
        .map_err(|e| crate::Error::Unknown(format!("Failed to create discussion: {e}")))?;

    Ok(post)
}
