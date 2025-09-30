use crate::models::space::{PollSpaceMetadata, PollSpaceResponse, PollSpaceSurvey, SpaceCommon};
use crate::types::{Partition, SpacePublishState, SpaceStatus, SurveyQuestion};
use crate::utils::dynamo_extractor::extract_user_from_session;
use crate::utils::security::{RatelResource, check_permission_from_session};
use crate::{AppState, Error2};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdatePollSpacePathParams {
    pub poll_space_pk: Partition,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpdatePollSpaceRequest {
    pub questions: Vec<SurveyQuestion>,
}

pub type UpdatePollSpaceResponse = PollSpaceResponse;

pub async fn update_poll_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Path(UpdatePollSpacePathParams { poll_space_pk }): Path<UpdatePollSpacePathParams>,
    Json(UpdatePollSpaceRequest { questions }): Json<UpdatePollSpaceRequest>,
) -> Result<Json<UpdatePollSpaceResponse>, Error2> {
    let user = extract_user_from_session(&dynamo.client, &session).await?;

    // Check Space Existence
    let space_common = SpaceCommon::get(
        &dynamo.client,
        &poll_space_pk,
        Some(crate::types::EntityType::SpaceCommon),
    )
    .await?
    .ok_or(Error2::NotFoundSpace)?;

    // Check Permissions
    match space_common.user_pk {
        Partition::User(_) => {
            if user.pk != space_common.user_pk {
                return Err(Error2::Unauthorized(
                    "No permission to update this poll space".to_string(),
                ));
            }
        }
        Partition::Team(_) => {
            check_permission_from_session(
                &dynamo.client,
                &session,
                RatelResource::Team {
                    team_pk: space_common.user_pk.to_string(),
                },
                vec![crate::types::TeamGroupPermission::SpaceEdit],
            )
            .await?;
        }
        _ => {
            return Err(Error2::InternalServerError(
                "Invalid user_pk in space_common".to_string(),
            ));
        }
    }

    // Check Space Status
    // Only publish_state is Draft or Waiting status can be updated
    if (space_common.publish_state == SpacePublishState::Published
        && space_common.status != Some(SpaceStatus::Waiting))
        || space_common.publish_state != SpacePublishState::Draft
    {
        return Err(Error2::ImmutablePollSpaceState);
    }

    PollSpaceSurvey::new(poll_space_pk.clone(), questions)
        .create(&dynamo.client)
        .await?;

    let poll_metadata = PollSpaceMetadata::query(&dynamo.client, &poll_space_pk).await?;
    let response = PollSpaceResponse::from(poll_metadata);
    Ok(Json(response))
}
