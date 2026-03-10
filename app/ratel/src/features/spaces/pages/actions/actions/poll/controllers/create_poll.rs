use crate::features::spaces::pages::actions::actions::poll::*;

#[post("/api/spaces/{space_pk}/polls", role: SpaceUserRole)]
pub async fn create_poll(space_pk: SpacePartition) -> Result<PollResponse> {
    SpacePoll::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let poll = SpacePoll::new(space_pk.clone())?;

    let space_pk = space_pk.into();
    let _ =
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::get_or_create(cli, &space_pk).await?;

    let mut items = vec![poll.create_transact_write_item()];
    items.push(crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_polls(&space_pk, 1));
    crate::transact_write_items!(cli, items)
        .map_err(|e| crate::features::spaces::pages::actions::actions::poll::Error::Unknown(format!("Failed to create poll: {e}")))?;

    Ok(poll.into())
}
