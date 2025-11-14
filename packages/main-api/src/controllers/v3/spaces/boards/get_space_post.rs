#![allow(warnings)]
use crate::{
    AppState, Error, Permissions,
    controllers::v3::spaces::{SpacePath, SpacePathParam, SpacePostPath, SpacePostPathParam},
    features::spaces::boards::{
        dto::{
            space_post_comment_response::SpacePostCommentResponse,
            space_post_response::SpacePostResponse,
        },
        models::{
            space_category::SpaceCategory,
            space_post::SpacePost,
            space_post_comment::{SpacePostComment, SpacePostCommentQueryOption},
            space_post_comment_like::SpacePostCommentLike,
        },
    },
    models::{SpaceCommon, feed::Post, team::Team, user::User},
    types::{EntityType, Partition, TeamGroupPermission, author::Author},
};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

pub async fn get_space_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
) -> Result<Json<SpacePostResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);
    let post = SpacePost::get(&dynamo.client, pk, Some(sk))
        .await?
        .ok_or(Error::PostNotFound)?;

    //FIXME: remove this logic
    let (comments, _bookmark) = SpacePostComment::find_by_post_order_by_likes(
        &dynamo.client,
        space_post_pk.clone(),
        SpacePostComment::opt_all().scan_index_forward(false),
    )
    .await?;

    tracing::debug!("comments (sorted by likes desc): {:?}", comments);

    let comments: Vec<_> = comments
        .into_iter()
        .filter(|c| matches!(c.sk, EntityType::SpacePostComment(_)))
        .collect();

    let mut comment_keys = Vec::with_capacity(comments.len());
    for c in &comments {
        comment_keys.push(c.like_keys(&user.pk));
    }

    let comment_likes = SpacePostCommentLike::batch_get(&dynamo.client, comment_keys).await?;

    let mut comment_res = Vec::with_capacity(comments.len());

    for comment in comments {
        let liked = comment_likes.iter().any(|like| like == &comment);

        let mut c: SpacePostCommentResponse = comment.into();
        c.liked = liked;

        comment_res.push(c);
    }

    let mut post: SpacePostResponse = post.into();
    post.comments = comment_res;

    Ok(Json(post))
}
