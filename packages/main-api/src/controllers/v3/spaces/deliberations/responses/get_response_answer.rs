use crate::{
    AppState, Error2,
    models::{
        space::{DeliberationSpaceResponse, SpaceCommon},
        user::User,
    },
    types::{EntityType, Partition, TeamGroupPermission},
};
use aide::NoApi;
use bdk::prelude::axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct GetResponseAnswerRequest {
    #[schemars(description = "Survey ID")]
    pub survey_id: String,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationResponseByIdPath {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub response_pk: Partition,
}

pub async fn get_response_answer_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(DeliberationResponseByIdPath {
        space_pk,
        response_pk,
    }): Path<DeliberationResponseByIdPath>,
) -> Result<Json<DeliberationSpaceResponse>, Error2> {
    let id = match response_pk {
        Partition::SurveyResponse(v) => v,
        _ => "".to_string(),
    };
    let response = DeliberationSpaceResponse::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationResponse(id.to_string())),
    )
    .await?;

    if response.is_none() {
        Err(Error2::NotFound("Response not found".to_string()))?;
    }

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

    let response = response.unwrap();

    Ok(Json(response))
}
