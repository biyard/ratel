use super::dto::*;
use crate::models::feed::Post;
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::types::{BoosterType, Partition, SpaceType, TeamGroupPermission};
use crate::{AppState, Error2};
use aide::NoApi;
use axum::extract::{Json, State};
use bdk::prelude::*;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
// pub struct CreateSpacePathParams {
//     post_pk: Partition,
// }

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceRequest {
    pub(crate) space_type: SpaceType,
    pub(crate) post_pk: Partition,

    time_range: Option<TimeRange>,
    booster: Option<BoosterType>,
}

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceResponse {
    pub space_pk: Partition,
}

pub async fn create_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(CreateSpaceRequest {
        space_type,
        post_pk,
        time_range,
        booster,
    }): Json<CreateSpaceRequest>,
) -> Result<Json<CreateSpaceResponse>, Error2> {
    let (post, has_perm) = Post::has_permission(
        &dynamo.client,
        &post_pk,
        Some(&user.pk),
        TeamGroupPermission::PostEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let (started_at, ended_at) = if let Some(tr) = &time_range {
        if !tr.is_valid() {
            return Err(Error2::InvalidTimeRange);
        }
        (Some(tr.0), Some(tr.1))
    } else {
        (None, None)
    };
    let mut space = SpaceCommon::new(post_pk, user.clone());

    let mut post_updater = Post::updater(&post.pk, &post.sk)
        .with_space_pk(space.pk.clone())
        .with_space_type(space_type);

    if started_at.is_some() && ended_at.is_some() {
        space = space.with_time(started_at.unwrap(), ended_at.unwrap());
    }
    if booster.is_some() {
        space = space.with_booster(booster.unwrap());
        post_updater = post_updater.with_booster(booster.unwrap());
    }
    let space_tx = space.create_transact_write_item();
    let post_tx = post_updater.transact_write_item();

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(vec![space_tx, post_tx]))
        .send()
        .await
        .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;

    Ok(Json(CreateSpaceResponse { space_pk: space.pk }))
}
