use crate::*;

#[get("/api/spaces/{space_pk}/discussions/{discussion_sk}", role: SpaceUserRole)]
pub async fn get_discussion(
    space_pk: SpacePartition,
    discussion_sk: SpacePostEntityType,
) -> Result<DiscussionResponse> {
    SpacePost::can_view(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let post = SpacePost::get(cli, &space_pk, Some(discussion_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Discussion not found".into()))?;

    let response: DiscussionResponse = post.into();
    Ok(response)
}
