use crate::{
    AppState, Error2,
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
) -> Result<Json<PostDetailResponse>, Error2> {
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

    let post = post.ok_or(Error2::PostNotFound)?;

    let permissions = post.get_permissions(cli, user.clone()).await?;
    if !permissions.contains(crate::types::TeamGroupPermission::PostRead) {
        return Err(Error2::Unauthorized(
            "You do not have permission to view this post".into(),
        ));
    }

    let (is_liked, comment_likes) = if let Some(user) = &user {
        let is_liked = post.is_liked(cli, &user.pk);
        let comment_likes = PostCommentLike::batch_get(cli, comment_keys);
        let ret = tokio::try_join!(is_liked, comment_likes)?;

        ret
    } else {
        (false, vec![])
    };

    // TODO: query with sk
    // let post_likes = PostLikeMetadata::query(cli, &post_like_pk);

    // let (post_metadata, post_likes) = tokio::try_join!(post_metadata, post_likes)?;

    // TODO: Check if the user has liked the post and set is_liked accordingly

    Ok(Json(
        (post_metadata, permissions.into(), is_liked, comment_likes).into(),
    ))
}
