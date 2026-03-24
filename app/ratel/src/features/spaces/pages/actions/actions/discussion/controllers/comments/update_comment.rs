use crate::common::models::space::SpaceCommon;
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}", role: SpaceUserRole, user: crate::features::auth::User, space: SpaceCommon)]
pub async fn update_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentTargetEntityType,
    req: UpdateCommentRequest,
) -> Result<SpacePostComment> {
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
        space.join_anytime,
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

    let space_post_pk: Partition = space_post_id.into();

    let comment_sk_entity: EntityType = comment_sk.into();

    let comment = SpacePostComment::get(cli, &space_post_pk, Some(comment_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Comment not found".into()))?;

    // Only the author can update
    if comment.author_pk != user.pk {
        return Err(Error::NoPermission);
    }

    let now = chrono::Utc::now().timestamp();
    let comment = SpacePostComment::updater(&space_post_pk, &comment_sk_entity)
        .with_content(req.content)
        .with_updated_at(now)
        .with_updated_at_align(format!("{:020}", now))
        .execute(cli)
        .await?;

    Ok(comment)
}
