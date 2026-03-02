use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LikeCommentRequest {
    pub like: bool,
}

#[post("/api/spaces/{space_pk}/discussions/{discussion_sk}/comments/{comment_sk}/likes", role: SpaceUserRole, user: ratel_auth::User)]
pub async fn like_comment(
    space_pk: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    req: LikeCommentRequest,
) -> Result<String> {
    SpacePost::can_view(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_post_pk = SpacePostPartition(discussion_sk.0.clone());

    let comment_sk_entity: EntityType = comment_sk.into();
    let user_pk: UserPartition = user
        .pk
        .clone()
        .try_into()
        .map_err(|_| Error::NoPermission)?;

    if req.like {
        SpacePost::like_comment(cli, space_post_pk, comment_sk_entity, user_pk).await?;
    } else {
        SpacePost::unlike_comment(cli, space_post_pk, comment_sk_entity, user_pk).await?;
    }

    Ok("success".to_string())
}
