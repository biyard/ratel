use crate::{
    AppState, Error2,
    models::space::{DeliberationDetailResponse, DeliberationMetadata, SpaceCommon},
    types::{Partition, TeamGroupPermission},
    utils::dynamo_extractor::extract_user_from_session,
};
use bdk::prelude::axum::{
    Extension,
    extract::{Json, Path, State},
};
use bdk::prelude::*;
use tower_sessions::Session;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationGetPath {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

pub async fn get_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationGetPath { space_pk }): Path<DeliberationGetPath>,
) -> Result<Json<DeliberationDetailResponse>, Error2> {
    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;
    let user = extract_user_from_session(&dynamo.client, &session).await?;
    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    tracing::debug!("Deliberation metadata retrieved: {:?}", metadata);
    let mut metadata: DeliberationDetailResponse = metadata.into();

    tracing::debug!("DeliberationDetailResponse formed: {:?}", metadata);
    let responses = metadata.clone().surveys.responses;

    for response in responses {
        if response.user_pk == user.pk {
            metadata.surveys.user_responses.push(response);
        }
    }

    Ok(Json(metadata))
}
