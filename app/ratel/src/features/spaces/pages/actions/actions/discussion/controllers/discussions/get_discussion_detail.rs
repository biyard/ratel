use crate::features::spaces::pages::actions::actions::discussion::*;

#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/detail", role: SpaceUserRole, user: crate::common::models::OptionalUser)]
pub async fn get_discussion_detail(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
) -> Result<DiscussionResponse> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_id.clone().into();
    let discussion_sk_entity: EntityType = discussion_sk.clone().into();

    let post = SpacePost::get(cli, &space_pk, Some(discussion_sk_entity.clone()))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;

    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::get(
        cli,
        &CompositePartition(space_id.clone(), discussion_sk.to_string()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::NotFound("SpaceAction not found".into()))?;

    let subscribed = if let Some(u) = &user.0 {
        let space_post_pk = match &discussion_sk_entity {
            EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
            _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
        };
        let (sub_pk, sub_sk) = SpacePostSubscription::keys(&space_post_pk, &u.pk);
        SpacePostSubscription::get(cli, &sub_pk, Some(sub_sk))
            .await
            .map_err(|e| {
                crate::error!("get_discussion_detail subscribed check failed: {e}");
                SpaceActionDiscussionError::NotFound
            })?
            .is_some()
    } else {
        false
    };

    Ok(DiscussionResponse {
        post,
        space_action,
        subscribed,
    })
}
