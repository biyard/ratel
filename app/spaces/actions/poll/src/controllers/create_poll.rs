use crate::*;

#[post("/api/spaces/{space_pk}/polls", role: SpaceUserRole)]
pub async fn create_poll(space_pk: SpacePartition) -> Result<PollResponse> {
    SpacePoll::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let poll = SpacePoll::new(space_pk)?;
    poll.create(cli).await?;

    Ok(poll.into())
}
