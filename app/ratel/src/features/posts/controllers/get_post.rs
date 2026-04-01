use std::collections::HashSet;

use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use crate::features::auth::OptionalUser;

#[mcp_tool(name = "get_post", description = "Get post details by ID.")]
#[get("/api/posts/:post_id", user: OptionalUser)]
pub async fn get_post_handler(post_id: FeedPartition) -> Result<PostDetailResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();
    let user: Option<crate::features::auth::User> = user.into();
    tracing::debug!("Get post for post_pk: {}", post_pk);

    let post_metadata = PostMetadata::query(cli, &post_pk).await?;
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

    let permissions = post.get_permissions(cli, user.clone()).await?;
    if !permissions.contains(TeamGroupPermission::PostRead) {
        return Err(Error::Unauthorized(
            "You do not have permission to view this post".into(),
        ));
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

    if !can_read_space {
        resp.post.as_mut().map(|p| {
            p.space_pk = None;
            p.space_type = None;
            p.space_visibility = None;
        });
    }

    Ok(resp)
}
