use crate::models::feed::Post;
use crate::models::space::{PollSpace, SpaceCommon};
use crate::types::{BoosterType, EntityType, Partition, SpaceType};
use crate::utils::dynamo_extractor::extract_user_from_session;
use crate::{AppState, Error2};

use axum::{
    Extension,
    extract::{Json, State},
};
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
    started_at: Option<i64>,
    ended_at: Option<i64>,
    booster: Option<BoosterType>,
}

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceResponse {
    pub space_pk: Partition,
}

pub async fn create_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Json(CreateSpaceRequest {
        space_type,
        post_pk,
        started_at,
        ended_at,
        booster,
    }): Json<CreateSpaceRequest>,
) -> Result<Json<CreateSpaceResponse>, Error2> {
    let user = extract_user_from_session(&dynamo.client, &session).await?;
    // FIXME: Check Post Visibility
    let post = Post::get(&dynamo.client, &post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::PostNotFound)?;
    let space_pk = match space_type {
        SpaceType::Poll => {
            let space = PollSpace::new();
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
