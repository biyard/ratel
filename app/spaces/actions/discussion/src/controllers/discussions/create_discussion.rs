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

    let space_pk = space_id.into();
    let _ =
        space_common::models::aggregate::DashboardAggregate::get_or_create(cli, &space_pk).await?;

    let mut items = vec![post.create_transact_write_item()];
    items.push(space_common::models::aggregate::DashboardAggregate::inc_posts(&space_pk, 1));
    transact_write_items!(cli, items)
        .map_err(|e| crate::Error::Unknown(format!("Failed to create discussion: {e}")))?;

    Ok(post)
}
