use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::{common::types::ListResponse, features::auth::OptionalUser};
use std::collections::HashSet;

#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/comments/{comment_sk}/replies?bookmark", role: SpaceUserRole, user: OptionalUser)]
pub async fn list_replies(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    comment_sk: SpacePostCommentEntityType,
    bookmark: Option<String>,
) -> Result<ListResponse<DiscussionCommentResponse>> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let discussion_sk_entity: EntityType = discussion_sk.into();
    let space_post_pk: SpacePostPartition = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(Error::BadRequest("Invalid discussion id".into())),
    };
    let comment_sk_entity: EntityType = comment_sk.into();

    let opt = SpacePostComment::opt_all()
        .scan_index_forward(false)
        .limit(50);
    let opt = if let Some(b) = bookmark {
        opt.bookmark(b)
    } else {
        opt
    };

    let (replies, next_bookmark) =
        SpacePostComment::list_by_comment(cli, comment_sk_entity, opt).await?;

    let space_post_pk: Partition = space_post_pk.into();
    let liked_set: HashSet<String> = if let Some(ref u) = user.0 {
        let keys: Vec<_> = replies.iter().map(|r| r.like_keys(&u.pk)).collect();
        let likes = SpacePostCommentLike::batch_get(cli, keys)
            .await
            .unwrap_or_default();
        likes.into_iter().map(|l| l.sk.to_string()).collect()
    } else {
        HashSet::new()
    };

    let responses: Vec<DiscussionCommentResponse> = replies
        .into_iter()
        .map(|r| {
            let like_key = if let Some(ref u) = user.0 {
                let (_, sk) = SpacePostCommentLike::keys_from_partition(
                    space_post_pk.clone(),
                    r.sk.clone(),
                    &u.pk,
                );
                sk.to_string()
            } else {
                String::new()
            };
            let liked = liked_set.contains(&like_key);
            (r, liked).into()
        })
        .collect();

    Ok(ListResponse {
        items: responses,
        bookmark: next_bookmark,
    })
}
