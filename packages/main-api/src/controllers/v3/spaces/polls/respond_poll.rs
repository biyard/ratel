use crate::controllers::v3::spaces::dto::*;
use crate::models::space::SpaceCommon;

use crate::features::spaces::polls::{Poll, PollQuestion, PollStatus, PollUserAnswer};
use crate::models::user::User;
use crate::types::{Answer, EntityType, Partition, TeamGroupPermission, validate_answers};
use crate::{AppState, Error};

use aide::NoApi;

use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct RespondPollSpaceRequest {
    answers: Vec<Answer>,
}

#[derive(Debug, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct RespondPollSpaceResponse {
    poll_space_pk: Partition,
}

pub async fn respond_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): aide::NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<RespondPollSpaceRequest>,
) -> crate::Result<Json<RespondPollSpaceResponse>> {
    //Validate Request

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let poll = Poll::get(&dynamo.client, &space_pk, Some(EntityType::Space))
        .await?
        .ok_or(Error::NotFoundPoll)?;

    // Space Status Check
    if poll.status != PollStatus::InProgress {
        return Err(Error::PollNotInProgress);
    }

    //
    // if poll.response_editable

    //Validate Answers
    let poll_question = PollQuestion::get(
        &dynamo.client,
        &space_pk,
        Some(EntityType::SpacePollQuestion),
    )
    .await?
    .ok_or(Error::NotFoundPoll)?;

    if !validate_answers(poll_question.questions, req.answers.clone()) {
        return Err(Error::PollAnswersMismatchQuestions);
    }

    let mut transact_items = vec![];

    if let Some(response) = PollUserAnswer::find_one(&dynamo.client, &space_pk, &user.pk).await? {
        // Update existing response
        if !poll.response_editable {
            return Err(Error::ImmutablePollUserAnswer);
        }
        let update_tx = PollUserAnswer::updater(&response.pk, &response.sk)
            .with_answers(req.answers)
            .transact_write_item();
        transact_items.push(update_tx);
    } else {
        let create_tx = PollUserAnswer::new(poll.pk.clone(), user.pk.clone(), req.answers)
            .create_transact_write_item();
        transact_items.push(create_tx);

        let space_increment_tx = Poll::updater(&poll.pk, &poll.sk)
            .increase_user_response_count(1)
            .transact_write_item();
        transact_items.push(space_increment_tx);
    }

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(transact_items))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to respond poll space: {}", e);
            Error::InternalServerError("Failed to respond poll space".into())
        })?;
    Ok(Json(RespondPollSpaceResponse {
        poll_space_pk: space_pk,
    }))
}
