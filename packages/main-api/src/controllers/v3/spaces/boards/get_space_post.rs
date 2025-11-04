#![allow(warnings)]
use crate::{
    AppState, Error,
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
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
) -> Result<Json<SpacePostResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);
    let post = SpacePost::get(&dynamo.client, pk, Some(sk))
        .await?
        .unwrap_or_default();

    let mut bookmark = None::<String>;
    let mut comment_keys = vec![];
    let mut comments: Vec<SpacePostComment> = vec![];

    loop {
        let (responses, new_bookmark) = SpacePostComment::query(
            &dynamo.client,
            space_pk.clone(),
            if let Some(b) = &bookmark {
                SpacePostCommentQueryOption::builder()
                    .sk("SPACE_POST_COMMENT#".into())
                    .bookmark(b.clone())
            } else {
                SpacePostCommentQueryOption::builder().sk("SPACE_POST_COMMENT#".into())
            },
        )
        .await?;

        for response in responses {
            comment_keys.push(response.like_keys(&user.pk));
            comments.push(response.into());
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    let comment_likes = SpacePostCommentLike::batch_get(&dynamo.client, comment_keys).await?;
    let mut comment_res = vec![];

    for comment in comments {
        let liked = comment_likes.iter().any(|like| like == comment);
        let mut c: SpacePostCommentResponse = comment.into();
        c.liked = liked;

        comment_res.push(c);
    }

    let mut post: SpacePostResponse = post.into();
    post.comments = comment_res;

    Ok(Json(post))
}
