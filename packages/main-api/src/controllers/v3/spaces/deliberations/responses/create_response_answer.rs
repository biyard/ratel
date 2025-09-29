use crate::{
    AppState, Error2,
    models::space::{DeliberationDetailResponse, DeliberationMetadata, DeliberationSpaceResponse},
    types::{Partition, SurveyAnswer, SurveyType},
    utils::dynamo_extractor::extract_user,
};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};

use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct CreateResponseAnswerRequest {
    #[schemars(description = "Survey ID")]
    pub survey_id: String,
    #[schemars(description = "Survey Type(Sample, Survey)")]
    pub survey_type: SurveyType,
    #[schemars(description = "Survey Answers")]
    pub answers: Vec<SurveyAnswer>,
}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct CreateDeliberationResponse {
    pub metadata: DeliberationDetailResponse,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationResponsePath {
    pub deliberation_id: String,
}

pub async fn create_response_answer_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(DeliberationResponsePath { deliberation_id }): Path<DeliberationResponsePath>,
    Json(req): Json<CreateResponseAnswerRequest>,
) -> Result<Json<CreateDeliberationResponse>, Error2> {
    let user = extract_user(&dynamo.client, auth).await?;
    let deliberation_pk = Partition::DeliberationSpace(deliberation_id.to_string());
    let user_pk = match user.pk.clone() {
        Partition::User(v) => v,
        Partition::Team(v) => v,
        _ => "".to_string(),
    };

    let response = DeliberationSpaceResponse::new(
        deliberation_pk.clone(),
        Partition::Survey(req.survey_id.to_string()),
        req.survey_type,
        req.answers,
        user,
    );
    response.create(&dynamo.client).await?;

    let metadata = DeliberationMetadata::query(&dynamo.client, deliberation_pk.clone()).await?;
    let mut metadata: DeliberationDetailResponse = metadata.into();

    for res in &metadata.surveys.responses {
        if res.user_pk == user_pk {
            metadata.surveys.user_responses.push(res.clone());
            continue;
        }
    }

    Ok(Json(CreateDeliberationResponse { metadata }))
}
