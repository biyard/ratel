use crate::*;

#[delete("/api/spaces/{space_pk}/discussions/{discussion_sk}", role: SpaceUserRole)]
pub async fn delete_discussion(
    space_pk: SpacePartition,
    discussion_sk: SpacePostEntityType,
) -> Result<String> {
    SpacePost::can_edit(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    SpacePost::delete(cli, &space_pk, Some(discussion_sk_entity)).await?;
    // FIXME: If Category has no posts, delete the category
    Ok("success".to_string())
}
