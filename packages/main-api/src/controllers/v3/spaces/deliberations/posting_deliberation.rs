use crate::{
    AppState, Error2,
    controllers::v3::spaces::deliberations::update_deliberation::DeliberationPath,
    models::space::{DeliberationDetailResponse, DeliberationMetadata, SpaceCommon},
    types::EntityType,
    utils::dynamo_extractor::extract_user_from_session,
};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct PostingDeliberationRequest {}

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct PostingDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

pub async fn posting_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationPath { space_pk }): Path<DeliberationPath>,
    Json(_req): Json<PostingDeliberationRequest>,
) -> Result<Json<PostingDeliberationResponse>, Error2> {
    let space_pk = space_pk.replace("%23", "#");
    let user = extract_user_from_session(&dynamo.client, &session).await?;
    tracing::debug!("User extracted: {:?}", user);

    SpaceCommon::updater(&space_pk, EntityType::SpaceCommon)
        .with_status(dto::SpaceStatus::InProgress)
        .execute(&dynamo.client)
        .await?;

    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;

    let metadata: DeliberationDetailResponse = metadata.into();
    Ok(Json(PostingDeliberationResponse { metadata }))
}
