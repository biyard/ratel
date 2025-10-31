use crate::models::space::SpaceCommon;

use crate::features::spaces::polls::*;
use crate::models::user::User;
use crate::types::{Answer, EntityType, Partition, TeamGroupPermission, validate_answers};
use crate::utils::time::get_now_timestamp_millis;
use crate::{AppState, Error, transact_write};

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
    pub poll_space_pk: Partition,
}

pub async fn respond_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): aide::NoApi<User>,
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
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

    let poll_pk = match poll_sk.clone() {
        EntityType::SpacePoll(v) => Partition::Poll(v.to_string()),
        _ => Partition::Poll("".to_string()),
    };

    let poll = Poll::get(&dynamo.client, &space_pk, Some(&poll_sk))
        .await?
        .ok_or(Error::NotFoundPoll)?;

    // Space Status Check
    if poll.status() != PollStatus::InProgress {
        return Err(Error::PollNotInProgress);
    }

    if !validate_answers(poll.questions, req.answers.clone()) {
        return Err(Error::PollAnswersMismatchQuestions);
    }

    let user_response = PollUserAnswer::find_one(
        &dynamo.client,
        &poll.pk.clone(),
        &poll_pk.clone(),
        &user.pk.clone(),
    )
    .await?;

    if user_response.is_none() {
        let create_tx = PollUserAnswer::new(
            poll.pk.clone(),
            poll_pk.clone(),
            user.pk.clone(),
            req.answers,
        )
        .create_transact_write_item();

        let space_increment_tx = Poll::updater(&poll.pk, &poll.sk)
            .increase_user_response_count(1)
            .transact_write_item();

        transact_write!(&dynamo.client, create_tx, space_increment_tx)?;
    } else {
        let (pk, sk) = PollUserAnswer::keys(&user.pk.clone(), &poll_pk.clone(), &poll.pk.clone());
        let created_at = get_now_timestamp_millis();
        let _ = PollUserAnswer::updater(pk, sk)
            .with_answers(req.answers)
            .with_created_at(created_at)
            .execute(&dynamo.client)
            .await?;
    }

    Ok(Json(RespondPollSpaceResponse {
        poll_space_pk: space_pk,
    }))
}
