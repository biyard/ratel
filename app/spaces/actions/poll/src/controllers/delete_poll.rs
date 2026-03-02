use crate::*;
use ratel_auth::User;

#[delete("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole)]
pub async fn delete_poll(space_pk: SpacePartition, poll_sk: SpacePollEntityType) -> Result<String> {
    SpacePoll::can_edit(&role)?;
    let cli = common::CommonConfig::default().dynamodb();

    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let _poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    SpacePoll::delete(cli, &space_pk, Some(poll_sk_entity)).await?;

    Ok("success".to_string())
}
