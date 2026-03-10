use crate::features::spaces::pages::actions::actions::discussion::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyCommentRequest {
    pub content: String,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}/reply", role: SpaceUserRole, user: crate::features::auth::User)]
pub async fn reply_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    req: ReplyCommentRequest,
) -> Result<DiscussionCommentResponse> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let space_post_pk: SpacePostPartition = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(Error::BadRequest("Invalid discussion id".into())),
    };

    let comment_sk_entity: EntityType = comment_sk.into();

    let comment = SpacePostComment::reply(
        cli,
        space_id.clone(),
        space_post_pk,
        comment_sk_entity,
        req.content,
        &user,
    )
    .await?;

    let space_pk: Partition = space_id.into();
    let agg_item =
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_comments(&space_pk, 1);
    crate::transact_write_items!(cli, vec![agg_item]).ok();

    Ok(comment.into())
}
