use crate::{
    AppState, Error,
    models::{PostCommentLike, feed::PostMetadata, user::User},
};
use aide::NoApi;
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;

use super::*;

pub async fn get_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(PostPathParam { post_pk }): PostPath,
) -> Result<Json<PostDetailResponse>> {
    let cli = &dynamo.client;
    tracing::debug!("Get post for post_pk: {}", post_pk);

    let post_metadata = PostMetadata::query(cli, &post_pk).await?;
    let mut comment_keys = vec![];
    let mut post = None;

    for metadata in &post_metadata {
        match metadata {
            PostMetadata::PostComment(comment) => {
                if let Some(user) = &user {
                    comment_keys.push(comment.like_keys(&user.pk));
                }
            }
            PostMetadata::Post(p) => post = Some(p.clone()),
            _ => {}
        }
    }

    let post = post.ok_or(Error::PostNotFound)?;

    let permissions = post.get_permissions(cli, user.clone()).await?;
    if !permissions.contains(crate::types::TeamGroupPermission::PostRead) {
        return Err(Error::Unauthorized(
            "You do not have permission to view this post".into(),
        ));
    }
    let can_read_space = permissions.contains(crate::types::TeamGroupPermission::SpaceRead);

    let (is_liked, comment_likes) = if let Some(user) = &user {
        let is_liked = post.is_liked(cli, &user.pk);

        // DynamoDB batch_get_item has a limit of 100 items per request
        // Split comment_keys into chunks of 100 and process them
        let mut all_comment_likes = vec![];
        for chunk in comment_keys.chunks(100) {
            let chunk_likes = PostCommentLike::batch_get(cli, chunk.to_vec()).await?;
            all_comment_likes.extend(chunk_likes);
        }

        let is_liked = is_liked.await?;

        (is_liked, all_comment_likes)
    } else {
        (false, vec![])
    };

    // TODO: query with sk
    // let post_likes = PostLikeMetadata::query(cli, &post_like_pk);

    // let (post_metadata, post_likes) = tokio::try_join!(post_metadata, post_likes)?;

    // TODO: Check if the user has liked the post and set is_liked accordingly
    let mut resp: PostDetailResponse =
        (post_metadata, permissions.into(), is_liked, comment_likes).into();

    if !can_read_space {
        resp.post.as_mut().map(|p| {
            p.space_pk = None;
            p.space_type = None;
            p.space_visibility = None;
        });
    }

    Ok(Json(resp))
}
