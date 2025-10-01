use crate::{
    AppState, Error2,
    models::space::{
        DeliberationDetailResponse, DeliberationMetadata, DeliberationSpace,
        DeliberationSpaceResponse, SpaceCommon,
    },
    types::{
        EntityType, Partition, SpaceVisibility, SurveyAnswer, SurveyType, TeamGroupPermission,
    },
    utils::{
        dynamo_extractor::extract_user_from_session,
        security::{RatelResource, check_permission_from_session},
    },
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
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    pub space_pk: Partition,
}

pub async fn create_response_answer_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationResponsePath { space_pk }): Path<DeliberationResponsePath>,
    Json(req): Json<CreateResponseAnswerRequest>,
) -> Result<Json<CreateDeliberationResponse>, Error2> {
    let user = extract_user_from_session(&dynamo.client, &session).await?;
    let pk_id = match space_pk.clone() {
        Partition::DeliberationSpace(v) => v,
        _ => "".to_string(),
    };
    let survey_pk_id = match req.survey_pk {
        Partition::Survey(v) => v,
        _ => "".to_string(),
    };

    let space = DeliberationSpace::get(&dynamo.client, &space_pk, Some(EntityType::Space))
        .await?
        .ok_or(Error2::NotFound("Space not found".to_string()))?;

    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::NotFound("Space not found".to_string()))?;

    if space_common.visibility != SpaceVisibility::Public {
        let _ = match space.user_pk.clone() {
            Partition::Team(_) => {
                check_permission_from_session(
                    &dynamo.client,
                    &session,
                    RatelResource::Team {
                        team_pk: space.user_pk.to_string(),
                    },
                    vec![TeamGroupPermission::SpaceRead],
                )
                .await?;
            }
            Partition::User(_) => {
                let user = extract_user_from_session(&dynamo.client, &session).await?;
                if user.pk != space.user_pk {
                    return Err(Error2::Unauthorized(
                        "You do not have permission to create response answer".into(),
                    ));
                }
            }
            _ => return Err(Error2::InternalServerError("Invalid post author".into())),
        };
    }

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
