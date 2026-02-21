use crate::*;
use ratel_auth::User;

#[post("/api/polls/{space_pk}/{poll_sk}/delete", user: User)]
pub async fn delete_poll(space_pk: SpacePartition, poll_sk: SpacePollEntityType) -> Result<String> {
    let cli = crate::config::get().common.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let _poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;

    SpacePoll::delete(cli, &space_pk, Some(poll_sk_entity)).await?;

    Ok("success".to_string())
}
