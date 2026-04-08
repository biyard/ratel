use crate::features::spaces::pages::actions::actions::discussion::*;

#[mcp_tool(name = "get_discussion", description = "Get discussion details by ID.")]
#[get("/api/spaces/{space_id}/discussions/{discussion_sk}", role: SpaceUserRole)]
pub async fn get_discussion(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
) -> Result<SpacePost> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.into();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let post = SpacePost::get(cli, &space_pk, Some(discussion_sk_entity.clone()))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;

    Ok(post)
}
