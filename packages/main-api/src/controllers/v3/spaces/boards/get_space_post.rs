use crate::{
    AppState, Error, Permissions,
    controllers::v3::spaces::{SpacePath, SpacePathParam, SpacePostPath, SpacePostPathParam},
    features::{
        report::ContentReport,
        spaces::boards::{
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
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
) -> Result<Json<SpacePostResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);
    let post = SpacePost::get(&dynamo.client, pk, Some(sk))
        .await?
        .ok_or(Error::PostNotFound)?;

    let mut post_response: SpacePostResponse = post.clone().into();

    if user.is_some() {
        let is_report = ContentReport::is_reported_for_target_by_user(
            &dynamo.client,
            &post.clone().pk,
            Some(&post.clone().sk),
            &user.clone().unwrap().pk,
        )
        .await?;
        post_response.is_report = is_report;
    }

    Ok(Json(post_response))
}
