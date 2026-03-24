use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[delete("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}", role: SpaceUserRole, user : crate::features::auth::User, space: SpaceCommon)]
pub async fn delete_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentTargetEntityType,
) -> Result<()> {
    SpacePost::can_view(&role)?;

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_post_id = SpacePostPartition(discussion_sk.0.clone());
    let space_post_pk: Partition = space_post_id.clone().into();
    let discussion_sk_entity: EntityType = discussion_sk.clone().into();
    let space_action = SpaceAction::get(
        cli,
        &CompositePartition(space_id.clone(), discussion_sk.to_string()),
        Some(EntityType::SpaceAction),
    )
    .await?
    .ok_or(Error::SpaceActionNotFound)?;

    if !crate::features::spaces::pages::actions::can_execute_space_action(
        role,
        space_action.prerequisite,
        space.status,
        space.join_anytime,
    ) {
        return Err(Error::BadRequest(
            "Discussion is not available in the current space status".into(),
        ));
    }

    let (post_pk, post_sk) = SpacePost::keys(&space_id, &space_post_id);
    let post = SpacePost::get(cli, &post_pk, Some(post_sk.clone()))
        .await?
        .ok_or(Error::NotFound("Discussion not found".into()))?;
    if post.status() != DiscussionStatus::InProgress {
        return Err(Error::DiscussionNotInProgress);
    }
    let comment_sk_entity: EntityType = comment_sk.into();

    let comment = SpacePostComment::get(cli, &space_post_pk, Some(comment_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Comment not found".into()))?;

    if comment.author_pk != user.pk {
        return Err(Error::NoPermission);
    }

    let delete_comment_tx =
        SpacePostComment::delete_transact_write_item(&space_post_pk, &comment_sk_entity);

    let post_tx = SpacePost::updater(&post_pk, post_sk)
        .decrease_comments(1)
        .transact_write_item();

    let space_pk: Partition = space_id.into();
    let agg_item =
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_comments(
            &space_pk, -1,
        );

    let mut txs = vec![delete_comment_tx, post_tx, agg_item];

    if let Some(parent_sk) = &comment.parent_comment_sk {
        let parent_tx = SpacePostComment::updater(&space_post_pk, parent_sk)
            .decrease_replies(1)
            .transact_write_item();
        txs.push(parent_tx);
    }

    crate::transact_write_items!(cli, txs).map_err(|e| {
        tracing::error!("Failed to delete comment: {}", e);
        crate::features::spaces::pages::actions::actions::discussion::Error::Unknown(format!(
            "Failed to delete comment: {}",
            e
        ))
    })?;

    Ok(())
}
