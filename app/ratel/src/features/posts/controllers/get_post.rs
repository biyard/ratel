use std::collections::HashSet;

use crate::features::auth::OptionalUser;
use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;

#[mcp_tool(name = "get_post", description = "Get post details by ID.")]
#[get("/api/posts/:post_id", user: OptionalUser)]
pub async fn get_post_handler(post_id: FeedPartition) -> Result<PostDetailResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();
    let user: Option<crate::features::auth::User> = user.into();
    tracing::debug!("Get post for post_pk: {}", post_pk);

    // Restrict the query to sk prefix `POST` so cross-posting sidecars
    // (`SYNDICATION_DIRECTIVE`, `SYNDICATION_JOB#…`, `ENGAGEMENT_SNAPSHOT#…`)
    // colocated under the same pk don't get fetched and fail
    // `PostMetadata` (untagged enum of Post/PostComment/PostArtwork/PostRepost)
    // deserialization. All four variants' sk starts with `POST`.
    let post_metadata = PostMetadata::query_begins_with_sk(cli, &post_pk, "POST").await?;
    let mut comment_keys = vec![];
    let mut post = None;
    let mut post_comments = Vec::<PostComment>::new();

    for metadata in &post_metadata {
        match metadata {
            PostMetadata::PostComment(comment) => {
                if let Some(user) = &user {
                    comment_keys.push(comment.like_keys(&user.pk));
                }
                post_comments.push(comment.clone());
            }
            PostMetadata::Post(p) => post = Some(p.clone()),
            _ => {}
        }
    }

    let post = post.ok_or(Error::NotFound("Post not found".into()))?;

    // Sub-team broadcast / direct-message Posts are NEVER publicly readable.
    // The extension method short-circuits to Ok for ordinary posts and
    // resolves the broadcast audience (parent + relevant child team
    // members) for Broadcast-visibility ones. Defined in
    // `sub_team::services::broadcast_access`.
    use crate::features::sub_team::services::broadcast_access::PostBroadcastAccessExt;
    post.assert_broadcast_access(cli, user.as_ref().map(|u| &u.pk))
        .await?;

    let permissions = post.get_permissions(cli, user.clone()).await?;
    if !permissions.contains(TeamGroupPermission::PostRead) {
        return Err(PostError::NotAccessible.into());
    }
    let can_read_space = permissions.contains(TeamGroupPermission::SpaceRead);

    let (is_liked, comment_likes, reported_comment_ids) = if let Some(user) = &user {
        let is_liked = post.is_liked(cli, &user.pk).await?;

        let mut all_comment_likes = vec![];
        for chunk in comment_keys.chunks(100) {
            let chunk_likes = PostCommentLike::batch_get(cli, chunk.to_vec()).await?;
            all_comment_likes.extend(chunk_likes);
        }

        (is_liked, all_comment_likes, HashSet::new())
    } else {
        (false, vec![], HashSet::new())
    };

    let mut resp: PostDetailResponse = (
        post_metadata,
        permissions.into(),
        is_liked,
        false, // is_report - skipping ContentReport for now
        comment_likes,
        reported_comment_ids,
    )
        .into();

    // Direct role/ownership signals — preferred over the legacy `permissions`
    // bitmask on the client. Mirrors the Spaces pattern (server resolves the
    // viewer's role; client just reads it).
    if let Some(user) = &user {
        if post.user_pk == user.pk {
            resp.is_post_owner = true;
        }
        if matches!(post.user_pk, Partition::Team(_)) {
            resp.viewer_role = Team::get_user_role(cli, &post.user_pk, &user.pk).await?;
        }
    }

    if !can_read_space {
        resp.post.as_mut().map(|p| {
            p.space_pk = None;
            p.space_type = None;
            p.space_visibility = None;
        });
    }

    Ok(resp)
}
