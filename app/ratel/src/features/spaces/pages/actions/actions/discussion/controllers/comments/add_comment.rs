use crate::common::models::space::{SpaceAuthor, SpaceCommon};
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCommentRequest {
    pub content: String,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/comments", role: SpaceUserRole, author: SpaceAuthor, space: SpaceCommon)]
pub async fn add_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    req: AddCommentRequest,
) -> Result<DiscussionCommentResponse> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
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
    ) {
        return Err(Error::BadRequest(
            "Discussion is not available in the current space status".into(),
        ));
    }

    let space_post_id = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(Error::BadRequest("Invalid discussion id".into())),
    };
    let (post_pk, post_sk) = SpacePost::keys(&space_id, &space_post_id);
    let post = SpacePost::get(cli, &post_pk, Some(post_sk))
        .await?
        .ok_or(Error::NotFound("Discussion not found".into()))?;
    if post.status() != DiscussionStatus::InProgress {
        return Err(Error::DiscussionNotInProgress);
    }

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
