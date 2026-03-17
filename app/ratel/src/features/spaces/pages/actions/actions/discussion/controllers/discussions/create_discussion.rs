use crate::features::spaces::pages::actions::actions::discussion::*;

#[post("/api/spaces/{space_id}/discussions", role: SpaceUserRole, author: crate::common::models::space::SpaceAuthor)]
pub async fn create_discussion(space_id: SpacePartition) -> Result<SpacePost> {
    SpacePost::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let post = SpacePost::new(
        space_id.clone(),
        String::new(),
        String::new(),
        String::new(),
        &author,
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
        crate::features::spaces::pages::actions::actions::discussion::Error::Unknown(format!(
            "Failed to create discussion: {e}"
        ))
    })?;

    Ok(post)
}
