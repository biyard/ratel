use crate::features::spaces::pages::actions::actions::discussion::*;

#[mcp_tool(
    name = "create_discussion",
    description = "Create a new discussion in a space."
)]
#[post("/api/spaces/{space_id}/discussions", role: SpaceUserRole, member: crate::common::models::space::SpaceUser, _space: crate::common::models::space::SpaceCommon)]
pub async fn create_discussion(
    #[mcp(description = "Space partition key (e.g. 'SPACE#<uuid>')")] space_id: SpacePartition,
) -> Result<SpacePost> {
    SpacePost::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let post = SpacePost::new(
        space_id.clone(),
        String::new(),
        String::new(),
        String::new(),
        &member,
        None,
        None,
    );

    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::new(
        space_id.clone(),
        SpacePostEntityType::from(post.sk.clone()).to_string(),
        crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
    );
    let space_pk: Partition = space_id.into();
    let _ =
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::get_or_create(cli, &space_pk).await?;

    let mut items = vec![
        post.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    items.push(
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_posts(
            &space_pk, 1,
        ),
    );
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::error!("Failed to create discussion: {e}");
        SpaceActionDiscussionError::CreateFailed
    })?;

    crate::features::spaces::space_common::services::bump_participant_activity(
        cli, &space_pk, &member.pk,
    )
    .await;

    Ok(post)
}
