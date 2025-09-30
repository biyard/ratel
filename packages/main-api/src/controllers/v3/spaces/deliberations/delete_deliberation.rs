use crate::{
    AppState, Error2,
    models::space::{
        DeliberationMetadata, DeliberationSpace, DeliberationSpaceContent,
        DeliberationSpaceDiscussion, DeliberationSpaceElearning, DeliberationSpaceMember,
        DeliberationSpaceParticipant, DeliberationSpaceQuestion, DeliberationSpaceResponse,
        DeliberationSpaceSurvey, SpaceCommon,
    },
};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct DeleteDeliberationResponse {
    pub space_pk: String,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationDeletePath {
    pub space_pk: String,
}

pub async fn delete_deliberation_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(_session): Extension<Session>,
    Path(DeliberationDeletePath { space_pk }): Path<DeliberationDeletePath>,
) -> Result<Json<DeleteDeliberationResponse>, Error2> {
    let space_pk = space_pk.replace("%23", "#");
    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;

    for data in metadata.into_iter() {
        match data {
            DeliberationMetadata::DeliberationSpace(v) => {
                DeliberationSpace::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceSurvey(v) => {
                DeliberationSpaceSurvey::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceContent(v) => {
                DeliberationSpaceContent::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceResponse(v) => {
                DeliberationSpaceResponse::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceQuestion(v) => {
                DeliberationSpaceQuestion::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceParticipant(v) => {
                DeliberationSpaceParticipant::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceMember(v) => {
                DeliberationSpaceMember::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceElearning(v) => {
                DeliberationSpaceElearning::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::DeliberationSpaceDiscussion(v) => {
                DeliberationSpaceDiscussion::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
            DeliberationMetadata::SpaceCommon(v) => {
                SpaceCommon::delete(&dynamo.client, v.pk, Some(v.sk)).await?;
            }
        }
    }

    Ok(Json(DeleteDeliberationResponse { space_pk }))
}
