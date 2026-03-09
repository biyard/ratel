use crate::*;

#[post("/api/spaces/{space_pk}/polls", role: SpaceUserRole)]
pub async fn create_poll(space_pk: SpacePartition) -> Result<PollResponse> {
    SpacePoll::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let poll = SpacePoll::new(space_pk.clone())?;

    let space_pk = space_pk.into();
    let _ =
        space_common::models::aggregate::DashboardAggregate::get_or_create(cli, &space_pk).await?;

    let mut items = vec![poll.create_transact_write_item()];
    items.push(space_common::models::aggregate::DashboardAggregate::inc_polls(&space_pk, 1));
    transact_write_items!(cli, items)
        .map_err(|e| crate::Error::Unknown(format!("Failed to create poll: {e}")))?;

    Ok(poll.into())
}
