use crate::features::auth::OptionalUser;
use crate::features::spaces::pages::actions::actions::discussion::*;

// Single-comment fetch backing the dedicated replies page — the mobile
// "reply" navigation lands there cold (no cached thread state), so it
// needs to pull the parent on its own without refetching the whole
// discussion page.
#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}", role: SpaceUserRole, user: OptionalUser)]
pub async fn get_comment(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
) -> Result<DiscussionCommentResponse> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let _ = space_id;

    let discussion_sk_entity: EntityType = discussion_sk.into();
    let space_post_pk: SpacePostPartition = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };
    let post_pk: Partition = space_post_pk.into();
    let comment_sk_entity: EntityType = comment_sk.into();

    let comment = SpacePostComment::get(cli, &post_pk, Some(comment_sk_entity))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;

    let liked = if let Some(ref u) = user.0 {
        let (like_pk, like_sk) = SpacePostCommentLike::keys_from_partition(
            post_pk.clone(),
            comment.sk.clone(),
            &u.pk,
        );
        SpacePostCommentLike::get(cli, &like_pk, Some(like_sk))
            .await
            .ok()
            .flatten()
            .is_some()
    } else {
        false
    };

    Ok((comment, liked).into())
}
