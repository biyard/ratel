use crate::models::space::{PollSpace, PollSpaceSurvey, PollSpaceSurveyResponse, SpaceCommon};

use crate::types::{EntityType, Partition, SpaceVisibility, SurveyAnswer, validate_answers};
use crate::utils::dynamo_extractor::extract_user_from_session;
use crate::utils::security::check_permission_from_session;
use crate::{AppState, Error2};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct RespondPollSpacePathParams {
    space_pk: Partition,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct RespondPollSpaceRequest {
    answers: Vec<SurveyAnswer>,
}

pub async fn respond_poll_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Path(RespondPollSpacePathParams { space_pk }): Path<RespondPollSpacePathParams>,
    Json(req): Json<RespondPollSpaceRequest>,
) -> Result<(), Error2> {
    // Authenticate User
    let user: crate::models::user::User =
        extract_user_from_session(&dynamo.client, &session).await?;

    // Space Status Check
    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error2::NotFoundSpace)?;
    match space_common.visibility {
        SpaceVisibility::Private => {
            if user.pk != space_common.user_pk {
                return Err(Error2::Unauthorized(
                    "No permission to access this private space".to_string(),
                ));
            }
        }
        SpaceVisibility::Team(team_pk) => {
            check_permission_from_session(
                &dynamo.client,
                &session,
                crate::utils::security::RatelResource::Team { team_pk },
                vec![crate::types::TeamGroupPermission::SpaceRead],
            )
            .await?;
        }
        _ => {}
    }
    if space_common.status != Some(crate::types::SpaceStatus::InProgress) {
        return Err(Error2::SpaceNotInProgress);
    }

    //Validate Request
    if !matches!(space_pk, Partition::PollSpace(_)) {
        return Err(Error2::NotFoundPollSpace);
    }

    let poll_space = PollSpace::get(&dynamo.client, &space_pk, Some(EntityType::PollSpace))
        .await?
        .ok_or(Error2::NotFoundPollSpace)?;

    //Validate Answers
    let poll_space_survey =
        PollSpaceSurvey::get(&dynamo.client, &space_pk, Some(EntityType::PollSpaceSurvey))
            .await?
            .ok_or(Error2::AnswersMismatchQuestions)?;

    if validate_answers(poll_space_survey.questions, req.answers.clone()) {
        return Err(Error2::AnswersMismatchQuestions);
    }

    PollSpaceSurveyResponse::new(space_pk.clone(), user.pk.clone(), req.answers)
        .create(&dynamo.client)
        .await?;
    PollSpace::updater(&poll_space.pk, &poll_space.sk)
        .increase_user_response_count(1)
        .execute(&dynamo.client)
        .await?;
    Ok(())
}
