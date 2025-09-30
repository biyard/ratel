use crate::{
    AppState, Error2,
    models::space::{DeliberationDetailResponse, DeliberationMetadata, DeliberationSpaceResponse},
    types::{Partition, SurveyAnswer, SurveyType},
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
pub struct CreateResponseAnswerRequest {
    #[schemars(description = "Survey ID")]
    pub survey_pk: String,
    #[schemars(description = "Survey Type(Sample, Survey)")]
    pub survey_type: SurveyType,
    #[schemars(description = "Survey Answers")]
    pub answers: Vec<SurveyAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationResponsePath {
    pub space_pk: String,
}

pub async fn create_response_answer_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationResponsePath { space_pk }): Path<DeliberationResponsePath>,
    Json(req): Json<CreateResponseAnswerRequest>,
) -> Result<Json<CreateDeliberationResponse>, Error2> {
    let space_pk = space_pk.replace("%23", "#");
    let user = extract_user_from_session(&dynamo.client, &session).await?;
    let pk_id = space_pk.split("#").last().unwrap_or_default().to_string();
    let survey_pk_id = req
        .survey_pk
        .split("#")
        .last()
        .unwrap_or_default()
        .to_string();

    let response = DeliberationSpaceResponse::new(
        Partition::DeliberationSpace(pk_id.to_string()),
        Partition::Survey(survey_pk_id.to_string()),
        req.survey_type,
        req.answers,
        user.clone(),
    );
    response.create(&dynamo.client).await?;

    let metadata = DeliberationMetadata::query(&dynamo.client, space_pk.clone()).await?;
    let mut metadata: DeliberationDetailResponse = metadata.into();

    for res in &metadata.surveys.responses {
        if res.user_pk == user.clone().pk {
            metadata.surveys.user_responses.push(res.clone());
            continue;
        }
    }

    Ok(Json(CreateDeliberationResponse { metadata }))
}
