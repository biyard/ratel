use ratel_auth::OptionalUser;

use crate::*;
use std::collections::HashSet;

#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/comments?bookmark", role: SpaceUserRole, user: OptionalUser)]
pub async fn list_comments(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
    bookmark: Option<String>,
) -> Result<Vec<DiscussionCommentResponse>> {
    SpacePost::can_view(&role)?;
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let space_post_pk: SpacePostPartition = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(Error::BadRequest("Invalid discussion id".into())),
    };

    let space_post_pk_p: Partition = space_post_pk.clone().into();

    let opt = SpacePostComment::opt_all()
        .sk(EntityType::SpacePostComment(String::default()).to_string())
        .scan_index_forward(false)
        .limit(50);
    let opt = if let Some(b) = bookmark {
        opt.bookmark(b)
    } else {
        opt
    };

    let (comments, _next_bookmark) =
        SpacePostComment::find_by_post_order_by_likes(cli, space_post_pk_p.clone(), opt).await?;

    // Check which comments the current user has liked
    let liked_set: HashSet<String> = if let Some(ref u) = user.0 {
        let keys: Vec<_> = comments.iter().map(|c| c.like_keys(&u.pk)).collect();
        let likes = SpacePostCommentLike::batch_get(cli, keys)
            .await
            .unwrap_or_default();
        likes.into_iter().map(|l| l.sk.to_string()).collect()
    } else {
        HashSet::new()
    };

    let responses: Vec<DiscussionCommentResponse> = comments
        .into_iter()
        .map(|c| {
            let like_key = if let Some(ref u) = user.0 {
                let (_, sk) = SpacePostCommentLike::keys_from_partition(
                    space_post_pk_p.clone(),
                    c.sk.clone(),
                    &u.pk,
                );
                sk.to_string()
            } else {
                String::new()
            };
            let liked = liked_set.contains(&like_key);
            (c, liked).into()
        })
        .collect();

    Ok(responses)
}
