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
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    poll_space_pk: Partition,
}

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct RespondPollSpaceRequest {
    answers: Vec<SurveyAnswer>,
}

pub async fn respond_poll_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Path(RespondPollSpacePathParams { poll_space_pk }): Path<RespondPollSpacePathParams>,
    Json(req): Json<RespondPollSpaceRequest>,
) -> Result<(), Error2> {
    // Authenticate User
    let user: crate::models::user::User =
        extract_user_from_session(&dynamo.client, &session).await?;

    // Space Status Check
    let space_common = SpaceCommon::get(
        &dynamo.client,
        &poll_space_pk,
        Some(EntityType::SpaceCommon),
    )
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
    if !matches!(poll_space_pk, Partition::PollSpace(_)) {
        return Err(Error2::NotFoundPollSpace);
    }

    let poll_space = PollSpace::get(&dynamo.client, &poll_space_pk, Some(EntityType::Space))
        .await?
        .ok_or(Error2::NotFoundPollSpace)?;

    //Validate Answers
    let poll_space_survey = PollSpaceSurvey::get(
        &dynamo.client,
        &poll_space_pk,
        Some(EntityType::PollSpaceSurvey),
    )
    .await?
    .ok_or(Error2::NotFoundPollSpace)?;

    if !validate_answers(poll_space_survey.questions, req.answers.clone()) {
        return Err(Error2::AnswersMismatchQuestions);
    }
    let existing_response = PollSpaceSurveyResponse::get(
        &dynamo.client,
        Partition::PollSpaceResponse(user.pk.to_string()),
        Some(EntityType::PollSpaceSurveyResponse(
            poll_space_pk.to_string(),
        )),
    )
    .await?;

    PollSpaceSurveyResponse::new(poll_space_pk.clone(), user.pk.clone(), req.answers)
        .create(&dynamo.client)
        .await?;

    if existing_response.is_none() {
        PollSpace::updater(&poll_space.pk, &poll_space.sk)
            .increase_user_response_count(1)
            .execute(&dynamo.client)
            .await?;
    }

    Ok(())
}
