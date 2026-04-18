use crate::common::types::ListResponse;
use crate::features::auth::OptionalUser;

use crate::features::spaces::pages::actions::actions::discussion::*;
use std::collections::HashSet;

#[mcp_tool(name = "list_comments", description = "List comments on a discussion, sorted by likes. Supports pagination. Pass `since` (unix seconds) to fetch only comments created after that time, ordered newest-first with no pagination.")]
#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/comments?bookmark&since", role: SpaceUserRole, user: OptionalUser)]
pub async fn list_comments(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
    #[mcp(description = "Pagination bookmark. Omit for first page.")]
    bookmark: Option<String>,
    #[mcp(description = "Unix seconds timestamp. When set, returns only top-level comments with created_at > since, newest-first, no pagination.")]
    since: Option<i64>,
) -> Result<ListResponse<DiscussionCommentResponse>> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let space_post_pk: SpacePostPartition = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };
    let space_post_pk_p: Partition = space_post_pk.clone().into();

    // Polling branch: fetch only top-level comments newer than `since`, ordered
    // newest-first. Uses the base table (PK=post, SK begins_with SPACE_POST_COMMENT#)
    // with `scan_index_forward=false`; SK is a UUID v7 which is lexicographically
    // time-ordered, so descending SK order matches descending `created_at`. This
    // lets us early-terminate via `take_while` once we pass the `since` cutoff.
    let (comments, next_bookmark): (Vec<SpacePostComment>, Option<String>) =
        if let Some(since_ts) = since {
            let sk_prefix = EntityType::SpacePostComment(String::new()).to_string();
            let opt = SpacePostComment::opt()
                .sk(sk_prefix)
                .scan_index_forward(false)
                .limit(50);
            let (items, _) =
                SpacePostComment::query(cli, space_post_pk_p.clone(), opt).await?;
            let filtered: Vec<_> = items
                .into_iter()
                .take_while(|c| c.created_at > since_ts)
                .collect();
            (filtered, None)
        } else {
            // Use GSI2 (find_by_post_order_by_likes) with ROOT_PARENT filter
            // This queries only top-level comments, already sorted by likes descending.
            let mut opt = SpacePostComment::opt().scan_index_forward(false).limit(50);
            if let Some(b) = bookmark {
                opt = opt.bookmark(b);
            }
            let (items, next) =
                SpacePostComment::find_by_post_order_by_likes(cli, space_post_pk_p.clone(), opt)
                    .await?;
            let filtered: Vec<_> = items
                .into_iter()
                .filter(|comment| comment.parent_id_for_likes == ROOT_PARENT)
                .take(50)
                .collect();
            (filtered, next)
        };

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

    Ok(ListResponse {
        items: responses,
        bookmark: next_bookmark,
    })
}
