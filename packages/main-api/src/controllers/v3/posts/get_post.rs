use crate::{
    AppState, Error2,
    models::{
        feed::{Post, PostDetailResponse, PostMetadata},
        user::User,
    },
};
use aide::NoApi;
use axum::{
    Json,
    extract::{Path, State},
};
use bdk::prelude::*;

pub async fn get_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(super::dto::PostPathParam { post_pk }): super::dto::PostPath,
) -> Result<Json<PostDetailResponse>, Error2> {
    let cli = &dynamo.client;
    tracing::info!("Get post for post_pk: {}", post_pk);

    if !Post::has_permission(
        cli,
        &post_pk,
        if let Some(ref user) = user {
            Some(&user.pk)
        } else {
            None
        },
        crate::types::TeamGroupPermission::PostRead,
    )
    .await?
    .1
    {
        return Err(Error2::NoPermission);
    };

    // let post_like_pk = match post_pk {
    //     Partition::Feed(ref post_pk) => Partition::PostLike(post_pk.clone()),
    //     None => Partition::None,
    // };

    let post_metadata = PostMetadata::query(cli, &post_pk).await?;
    // TODO: query with sk
    // let post_likes = PostLikeMetadata::query(cli, &post_like_pk);

    // let (post_metadata, post_likes) = tokio::try_join!(post_metadata, post_likes)?;

    // TODO: Check if the user has liked the post and set is_liked accordingly

    Ok(Json(post_metadata.into()))
}
