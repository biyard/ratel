use crate::models::feed::Post;
use crate::models::space::{DeliberationSpace, PollSpace, SpaceCommon, TimeRange};
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

    let space_pk = match space_type {
        SpaceType::Poll => {
            let space = PollSpace::new();
            space.create(&dynamo.client).await?;
            space.pk
        }
        SpaceType::Deliberation => {
            let space = DeliberationSpace::new();
            space.create(&dynamo.client).await?;
            space.pk
        }
        _ => {
            unimplemented!("Space type {:?} is not implemented yet", space_type)
        }
    };

    let mut space_common = SpaceCommon::new(space_pk.clone(), post_pk, user.clone());
    let mut post_updater = Post::updater(&post.pk, &post.sk).with_space_pk(space_pk.clone());
    if started_at.is_some() && ended_at.is_some() {
        space_common = space_common.with_time(started_at.unwrap(), ended_at.unwrap());
    }
    if booster.is_some() {
        space_common = space_common.with_booster(booster.unwrap());
        post_updater = post_updater.with_booster(booster.unwrap());
    }

    space_common.create(&dynamo.client).await?;
    post_updater.execute(&dynamo.client).await?;

    Ok(Json(CreateSpaceResponse { space_pk }))
}
