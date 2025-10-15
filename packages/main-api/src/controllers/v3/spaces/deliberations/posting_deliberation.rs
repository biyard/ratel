use crate::{
    AppState, Error2,
    controllers::v3::spaces::deliberations::update_deliberation::DeliberationPath,
    models::{
        User,
        space::{DeliberationDetailResponse, DeliberationMetadata, SpaceCommon},
    },
    types::{
        EntityType, Partition, SpacePublishState, SpaceStatus, SpaceVisibility, TeamGroupPermission,
    },
};
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use aide::NoApi;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct PostingDeliberationRequest {
    pub visibility: SpaceVisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct PostingDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

pub async fn posting_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
    Json(req): Json<PostingDeliberationRequest>,
) -> Result<Json<PostingDeliberationResponse>, Error2> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundDeliberationSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    SpaceCommon::updater(&space_pk, EntityType::SpaceCommon)
        .with_status(SpaceStatus::InProgress)
        .with_visibility(req.visibility)
        .with_publish_state(SpacePublishState::Published)
        .execute(&dynamo.client)
        .await?;
    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;
    let metadata: DeliberationDetailResponse = metadata.into();
    Ok(Json(PostingDeliberationResponse { metadata }))
}
