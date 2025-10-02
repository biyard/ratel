use crate::{
    AppState, Error2,
    models::space::{DeliberationSpaceResponse, SpaceCommon},
    types::{EntityType, Partition, SpaceVisibility, TeamGroupPermission},
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
use serde::Deserialize;
use tower_sessions::Session;
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
    Extension(session): Extension<Session>,
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
        Some(EntityType::DeliberationSpaceResponse(id.to_string())),
    )
    .await?;

    if response.is_none() {
        Err(Error2::NotFound("Response not found".to_string()))?;
    }

    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::NotFound("Space not found".to_string()))?;

    if space_common.visibility != SpaceVisibility::Public {
        let _ = match space_common.user_pk.clone() {
            Partition::Team(_) => {
                check_permission_from_session(
                    &dynamo.client,
                    &session,
                    RatelResource::Team {
                        team_pk: space_common.user_pk.to_string(),
                    },
                    vec![TeamGroupPermission::SpaceRead],
                )
                .await?;
            }
            Partition::User(_) => {
                let user = extract_user_from_session(&dynamo.client, &session).await?;
                if user.pk != space_common.user_pk {
                    return Err(Error2::Unauthorized(
                        "You do not have permission to get response answer".into(),
                    ));
                }
            }
            _ => {
                return Err(Error2::InternalServerError(
                    "Invalid deliberation author".into(),
                ));
            }
        };
    }

    let response = response.unwrap();

    Ok(Json(response))
}
