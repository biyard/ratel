use crate::{
    AppState, Error2,
    models::space::{DeliberationSpace, DeliberationSpaceResponse, SpaceCommon},
    types::{EntityType, Partition, SpaceVisibility, TeamGroupPermission},
    utils::{
        dynamo_extractor::extract_user_from_session,
        security::{RatelResource, check_permission_from_session},
    },
};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};

use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;
use tower_sessions::Session;
use urlencoding::decode;
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
    pub space_pk: String,
    pub response_pk: String,
}

const RESPONSE_PREFIX: &str = "SURVEY_RESPONSE#";

pub async fn get_response_answer_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Path(DeliberationResponseByIdPath {
        space_pk,
        response_pk,
    }): Path<DeliberationResponseByIdPath>,
) -> Result<Json<DeliberationSpaceResponse>, Error2> {
    let space_pk = decode(&space_pk).unwrap_or_default().to_string();
    let response_pk = decode(&response_pk).unwrap_or_default().to_string();
    let id = response_pk
        .strip_prefix(RESPONSE_PREFIX)
        .ok_or_else(|| Error2::BadRequest("Invalid response_pk format".into()))?
        .to_string();
    let response = DeliberationSpaceResponse::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::DeliberationSpaceResponse(id.to_string())),
    )
    .await?;

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
                        "You do not have permission to get response answer".into(),
                    ));
                }
            }
            _ => return Err(Error2::InternalServerError("Invalid post author".into())),
        };
    }

    if response.is_none() {
        Err(Error2::NotFound("Response not found".to_string()))?;
    }

    let response = response.unwrap();

    Ok(Json(response))
}
