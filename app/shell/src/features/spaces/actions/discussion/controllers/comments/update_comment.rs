use crate::features::spaces::actions::discussion::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}", role: SpaceUserRole, user: crate::features::auth::User)]
pub async fn update_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    req: UpdateCommentRequest,
) -> Result<SpacePostComment> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let space_post_pk = match &discussion_sk_entity {
        EntityType::SpacePost(id) => Partition::SpacePost(id.clone()),
        _ => return Err(Error::BadRequest("Invalid discussion id".into())),
    };

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
