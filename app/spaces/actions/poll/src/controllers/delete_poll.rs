use crate::*;

#[delete("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole)]
pub async fn delete_poll(space_pk: SpacePartition, poll_sk: SpacePollEntityType) -> Result<String> {
    SpacePoll::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let _poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    SpacePoll::delete(cli, &space_pk, Some(poll_sk_entity)).await?;

    SpacePoll::remove_dashboard(cli, &space_pk).await;

    Ok("success".to_string())
}
