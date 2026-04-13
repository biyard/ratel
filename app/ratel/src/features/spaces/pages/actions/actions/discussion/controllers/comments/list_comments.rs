use crate::common::types::ListResponse;
use crate::features::auth::OptionalUser;

use crate::features::spaces::pages::actions::actions::discussion::*;
use std::collections::HashSet;

#[mcp_tool(name = "list_comments", description = "List comments on a discussion, sorted by likes. Supports pagination.")]
#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/comments?bookmark", role: SpaceUserRole, user: OptionalUser)]
pub async fn list_comments(
    #[mcp(description = "Space partition key")]
    space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
    #[mcp(description = "Pagination bookmark. Omit for first page.")]
    bookmark: Option<String>,
) -> Result<ListResponse<DiscussionCommentResponse>> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let discussion_sk_entity: EntityType = discussion_sk.into();

    let space_post_pk: SpacePostPartition = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };

    // Use GSI3 (find_replies_by_likes) with ROOT_PARENT
    // This queries only top-level comments, already sorted by likes descending.
    let mut opt = SpacePostComment::opt().scan_index_forward(false).limit(50);
    if let Some(b) = bookmark {
        opt = opt.bookmark(b);
    }

    let space_post_pk_p: Partition = space_post_pk.clone().into();
    let (comments, next_bookmark) =
        SpacePostComment::find_by_post_order_by_likes(cli, space_post_pk_p.clone(), opt).await?;
    let comments: Vec<_> = comments
        .into_iter()
        .filter(|comment| comment.parent_id_for_likes == ROOT_PARENT)
        .take(50)
        .collect();

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
