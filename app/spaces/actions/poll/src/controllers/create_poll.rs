use crate::*;

#[post("/api/spaces/{space_pk}/polls", role: SpaceUserRole)]
pub async fn create_poll(space_pk: SpacePartition) -> Result<PollResponse> {
    SpacePoll::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let poll = SpacePoll::new(space_pk.clone())?;

    let mut items = vec![poll.create_transact_write_item()];
    items.extend(poll.dashboard_write_items());
    cli.transact_write_items()
        .set_transact_items(Some(items))
        .send()
        .await
        .map_err(|e| crate::Error::Unknown(format!("Failed to create poll: {e}")))?;

    Ok(poll.into())
}
