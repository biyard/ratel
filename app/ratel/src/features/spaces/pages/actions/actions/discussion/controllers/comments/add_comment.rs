use crate::features::spaces::pages::actions::actions::discussion::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCommentRequest {
    pub content: String,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/comments", role: SpaceUserRole, author: crate::common::models::space::SpaceAuthor)]
pub async fn add_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    req: AddCommentRequest,
) -> Result<DiscussionCommentResponse> {
    SpacePost::can_view(&role)?;
    if !matches!(role, SpaceUserRole::Creator | SpaceUserRole::Participant) {
        return Err(Error::NoPermission);
    }
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_post_id: SpacePostPartition = SpacePostPartition(discussion_sk.0.clone());

    let comment =
        SpacePost::comment(cli, space_id.clone(), space_post_id, req.content, &author).await?;

    let space_pk: Partition = space_id.into();
    let agg_item =
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_comments(
            &space_pk, 1,
        );
    crate::transact_write_items!(cli, vec![agg_item]).ok();

    Ok(comment.into())
}
