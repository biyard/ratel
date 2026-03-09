use crate::features::spaces::actions::discussion::*;

#[get("/api/spaces/{space_id}/discussions/{discussion_sk}", role: SpaceUserRole)]
pub async fn get_discussion(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
) -> Result<SpacePost> {
    SpacePost::can_view(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let post = SpacePost::get(cli, &space_pk, Some(discussion_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Discussion not found".into()))?;

    Ok(post)
}
