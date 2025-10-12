use crate::{
    AppState, Error2,
    models::{
        feed::{Post, PostMetadata},
        user::User,
    },
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
    let post = post_metadata
        .iter()
        .filter(|p| matches!(p, PostMetadata::Post(_)))
        .map(|p| {
            if let PostMetadata::Post(post) = p {
                Some(post.clone())
            } else {
                None
            }
        })
        .collect::<Vec<Option<Post>>>()
        .first()
        .ok_or(Error2::PostNotFound)?
        .clone()
        .ok_or(Error2::PostNotFound)?;

    let permissions = post.get_permissions(cli, user.clone()).await?;
    if !permissions.contains(crate::types::TeamGroupPermission::PostRead) {
        return Err(Error2::Unauthorized(
            "You do not have permission to view this post".into(),
        ));
    }

    let is_liked = if let Some(user) = &user {
        post.is_liked(cli, &user.pk).await?
    } else {
        false
    };

    // TODO: query with sk
    // let post_likes = PostLikeMetadata::query(cli, &post_like_pk);

    // let (post_metadata, post_likes) = tokio::try_join!(post_metadata, post_likes)?;

    // TODO: Check if the user has liked the post and set is_liked accordingly

    Ok(Json((post_metadata, permissions.into(), is_liked).into()))
}
