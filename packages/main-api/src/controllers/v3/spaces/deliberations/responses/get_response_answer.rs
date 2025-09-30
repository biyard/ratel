use crate::{AppState, Error2, models::space::DeliberationSpaceResponse, types::EntityType};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};

use dto::{JsonSchema, aide, schemars};
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
    pub deliberation_pk: String,
    pub response_pk: String,
}

pub async fn get_response_answer_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(_auth): Extension<Option<Authorization>>,
    Path(DeliberationResponseByIdPath {
        deliberation_pk,
        response_pk,
    }): Path<DeliberationResponseByIdPath>,
) -> Result<Json<DeliberationSpaceResponse>, Error2> {
    let id = response_pk
        .split("#")
        .last()
        .unwrap_or_default()
        .to_string();
    let response = DeliberationSpaceResponse::get(
        &dynamo.client,
        &deliberation_pk,
        Some(EntityType::DeliberationSpaceResponse(id.to_string())),
    )
    .await?;

    if response.is_none() {
        Err(Error2::NotFound("Response not found".to_string()))?;
    }

    let response = response.unwrap();

    Ok(Json(response))
}
