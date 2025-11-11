use crate::features::did::VerifiedAttributes;
use crate::features::spaces::panels::SpacePanelParticipant;

use crate::features::spaces::{SpaceParticipant, polls::*};
use crate::models::user::User;
use crate::types::{
    Age, Answer, CompositePartition, EntityType, Gender, Partition, TeamGroupPermission,
    validate_answers,
};
use crate::types::{RespondentAttr, SpaceStatus};
use crate::utils::time::get_now_timestamp_millis;
use crate::{AppState, Error, Permissions, transact_write};

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
    NoApi(permissions): NoApi<Permissions>,
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
    Json(req): Json<RespondPollSpaceRequest>,
) -> crate::Result<Json<RespondPollSpaceResponse>> {
    //Validate Request

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    // if space_common.status == Some(SpaceStatus::Started)
    //     || space_common.status == Some(SpaceStatus::Finished)
    // {
    //     return Err(Error::FinishedSpace);
    // }

    let poll_pk: Partition = poll_sk.clone().try_into()?;

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

    let participant =
        SpacePanelParticipant::get_participant_in_space(&dynamo.client, &space_pk, &user.pk).await;

    let mut respondent: Option<RespondentAttr> = None;

    if let Some(_p) = participant {
        let res = VerifiedAttributes::get(
            &dynamo.client,
            CompositePartition(user.pk.clone(), Partition::Attributes),
            None::<String>,
        )
        .await
        .unwrap_or_default()
        .unwrap_or(VerifiedAttributes::default());

        let age = if res.age().is_none() {
            None
        } else {
            match res.age().unwrap_or_default() {
                0..=17 => Some(Age::Range {
                    inclusive_max: 17,
                    inclusive_min: 0,
                }),
                18..=29 => Some(Age::Range {
                    inclusive_max: 29,
                    inclusive_min: 18,
                }),
                30..=39 => Some(Age::Range {
                    inclusive_max: 39,
                    inclusive_min: 30,
                }),
                40..=49 => Some(Age::Range {
                    inclusive_max: 49,
                    inclusive_min: 40,
                }),
                50..=59 => Some(Age::Range {
                    inclusive_max: 59,
                    inclusive_min: 50,
                }),
                60..=69 => Some(Age::Range {
                    inclusive_max: 69,
                    inclusive_min: 60,
                }),
                _ => Some(Age::Range {
                    inclusive_max: 100,
                    inclusive_min: 70,
                }),
            }
        };

        let gender = res.gender;
        let school = res.university;

        respondent = Some(RespondentAttr {
            age,
            gender,
            school,
        });
    }

    if user_response.is_none() {
        let create_tx = PollUserAnswer::new(
            poll.pk.clone(),
            poll_pk.clone(),
            user.pk.clone(),
            req.answers,
            respondent,
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
