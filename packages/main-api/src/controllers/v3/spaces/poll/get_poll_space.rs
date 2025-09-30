use crate::{
    AppState, Error2,
    models::space::{PollSpaceMetadata, PollSpaceResponse, PollSpaceSurveyResponse, SpaceCommon},
    types::{EntityType, Partition, SpaceVisibility, TeamGroupPermission},
    utils::{
        dynamo_extractor::extract_user_from_session,
        security::{RatelResource, check_permission_from_session},
    },
};
use dto::by_axum::axum::{
    Extension, Json,
    extract::{Path, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetPollSpacePathParams {
    pub space_pk: Partition,
}

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetPollSpaceQueryParams {}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct GetPollSpaceResponse {}

pub async fn get_poll_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Path(GetPollSpacePathParams { space_pk }): Path<GetPollSpacePathParams>,
) -> Result<Json<GetPollSpaceResponse>, Error2> {
    if !matches!(space_pk, Partition::PollSpace(_)) {
        return Err(Error2::NotFoundPollSpace);
    }
    let space = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::NotFoundSpace)?;
    match space.visibility {
        SpaceVisibility::Private => {
            let user = extract_user_from_session(&dynamo.client, &session).await?;
            if user.pk != space.user_pk {
                return Err(Error2::Unauthorized(
                    "No permission to access this private space".to_string(),
                ));
            }
        }
        SpaceVisibility::Team(team_pk) => {
            check_permission_from_session(
                &dynamo.client,
                &session,
                RatelResource::Team { team_pk },
                vec![TeamGroupPermission::SpaceRead],
            )
            .await?;
        }
        _ => {}
    }
    let metadata = PollSpaceMetadata::query(&dynamo.client, &space_pk).await?;
    let mut poll_space_response = PollSpaceResponse::from(metadata);
    if let Ok(user) = extract_user_from_session(&dynamo.client, &session).await {
        let my_survey_response = PollSpaceSurveyResponse::get(
            &dynamo.client,
            Partition::PollSpaceResponse(user.pk.to_string()),
            Some(EntityType::PollSpaceSurveyResponse(space_pk.to_string())),
        )
        .await?;

        poll_space_response.my_response = my_survey_response.map(|r| r.answers);
    }
    Ok(Json(GetPollSpaceResponse::default()))
}
