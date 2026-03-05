use crate::*;

#[delete("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}", role: SpaceUserRole, user : ratel_auth::User)]
pub async fn delete_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
) -> Result<()> {
    SpacePost::can_view(&role)?;

    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_post_id = SpacePostPartition(discussion_sk.0.clone());
    let space_post_pk: Partition = space_post_id.clone().into();
    let comment_sk_entity: EntityType = comment_sk.into();

    let comment = SpacePostComment::get(cli, &space_post_pk, Some(comment_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Comment not found".into()))?;

    if comment.author_pk != user.pk && role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let delete_comment_tx =
        SpacePostComment::delete_transact_write_item(&space_post_pk, &comment_sk_entity);

    let (pk, sk) = SpacePost::keys(&space_id, &space_post_id);
    let post_tx = SpacePost::updater(&pk, sk)
        .decrease_comments(1)
        .transact_write_item();

    let space_pk: Partition = space_id.into();
    let agg_item =
        space_common::models::dashboard::aggregate::DashboardAggregate::inc_comments(&space_pk, -1);

    let mut txs = vec![delete_comment_tx, post_tx, agg_item];

    if let Some(parent_sk) = &comment.parent_comment_sk {
        let parent_tx = SpacePostComment::updater(&space_post_pk, parent_sk)
            .decrease_replies(1)
            .transact_write_item();
        txs.push(parent_tx);
    }

    transact_write_items!(cli, txs).map_err(|e| {
        tracing::error!("Failed to delete comment: {}", e);
        crate::Error::Unknown(format!("Failed to delete comment: {}", e))
    })?;

    Ok(())
}
