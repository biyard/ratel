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
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct CreateResponseAnswerRequest {
    #[schemars(description = "Survey ID")]
    pub survey_pk: Partition,
    // #[schemars(description = "Survey Type(Sample, Survey)")]
    // pub survey_type: SurveyType,
    // #[schemars(description = "Survey Answers")]
    // pub answers: Vec<SurveyAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationResponsePath {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct DeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

pub async fn create_response_answer_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationResponsePath { space_pk }): Path<DeliberationResponsePath>,
    Json(req): Json<CreateResponseAnswerRequest>,
) -> Result<Json<DeliberationResponse>, Error2> {
    let user = extract_user_from_session(&dynamo.client, &session).await?;
    let survey_pk_id = match req.survey_pk {
        Partition::Survey(v) => v,
        _ => "".to_string(),
    };

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

    // let response = DeliberationSpaceResponse::new(
    //     space_pk.clone(),
    //     Partition::Survey(survey_pk_id.to_string()),
    //     req.survey_type,
    //     req.answers,
    //     user.clone(),
    // );
    // response.create(&dynamo.client).await?;

    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;
    let metadata: DeliberationDetailResponse = metadata.into();

    // for res in &metadata.surveys.responses {
    //     if res.user_pk == user.clone().pk {
    //         metadata.surveys.user_responses.push(res.clone());
    //         continue;
    //     }
    // }

    Ok(Json(DeliberationResponse { metadata }))
}
