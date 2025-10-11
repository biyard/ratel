use crate::models::space::{
    PollSpace, PollSpacePathParam, PollSpaceSurvey, PollSpaceSurveyResponse, SpaceCommon,
};

use crate::models::user::User;
use crate::types::{EntityType, Partition, SurveyAnswer, TeamGroupPermission, validate_answers};
use crate::{AppState, Error2};

use aide::NoApi;

use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct RespondPollSpaceRequest {
    answers: Vec<SurveyAnswer>,
}

pub async fn respond_poll_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): aide::NoApi<User>,
    Path(PollSpacePathParam { poll_space_pk }): Path<PollSpacePathParam>,
    Json(req): Json<RespondPollSpaceRequest>,
) -> Result<(), Error2> {
    //Validate Request

    let poll_space = PollSpace::get(&dynamo.client, &poll_space_pk, Some(EntityType::Space))
        .await?
        .ok_or(Error2::NotFoundPollSpace)?;

    let (space, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &poll_space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    // Space Status Check
    if space.status != Some(crate::types::SpaceStatus::InProgress) {
        return Err(Error2::SpaceNotInProgress);
    }

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

    let poll_space_response_tx =
        PollSpaceSurveyResponse::new(poll_space_pk.clone(), user.pk.clone(), req.answers)
            .create_transact_write_item();

    let mut transact_items = vec![poll_space_response_tx];
    if existing_response.is_none() {
        transact_items.push(
            PollSpace::updater(&poll_space.pk, &poll_space.sk)
                .increase_user_response_count(1)
                .transact_write_item(),
        );
    }

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(transact_items))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to respond poll space: {}", e);
            Error2::InternalServerError("Failed to respond poll space".into())
        })?;
    Ok(())
}
