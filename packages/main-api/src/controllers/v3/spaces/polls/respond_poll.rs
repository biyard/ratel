use crate::features::did::VerifiedAttributes;
use crate::features::spaces::panels::SpacePanelParticipant;

use crate::features::spaces::rewards::{
    PollRewardKey, RewardAction, RewardKey, SpaceReward, UserReward,
};
use crate::features::spaces::{SpaceDaoIncentiveScore, SpaceParticipant, polls::*};
use crate::models::user::User;
use crate::types::{
    Age, Answer, CompositePartition, EntityType, Gender, Partition, ResourcePermissions,
    TeamGroupPermission, validate_answers,
};
use crate::types::{RespondentAttr, SpaceStatus};
use crate::utils::time::get_now_timestamp_millis;
use crate::{AppState, Error, Permissions, transact_write};

use aide::NoApi;

use crate::models::SpaceCommon;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use by_axum::axum::Extension;
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
    State(AppState { dynamo, biyard, .. }): State<AppState>,
    NoApi(user): aide::NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
    Extension(space): Extension<SpaceCommon>,
    Json(req): Json<RespondPollSpaceRequest>,
) -> crate::Result<Json<RespondPollSpaceResponse>> {
    //Validate Request
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

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

    let is_admin = space.is_space_admin(&dynamo.client, &user).await;
    let is_participant = space.is_participant(&dynamo.client, &user.pk).await;
    if !is_participant && !is_admin {
        return Err(Error::UserNotParticipant);
    }
    if poll.status() != PollStatus::InProgress {
        return Err(Error::PollNotInProgress);
    }

    if !validate_answers(poll.clone().questions, req.answers.clone()) {
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
        let attribute = VerifiedAttributes::get_attributes(&dynamo, user.pk.clone()).await?;
        respondent = attribute;
    }

    // Response Poll Reward
    let reward = SpaceReward::get_by_reward_key(
        &dynamo.client,
        space_pk.clone().into(),
        (poll.sk.clone().into(), PollRewardKey::Respond).into(),
    )
    .await;

    // Create or Update User Response

    if user_response.is_none() {
        let user_pk = user.pk.clone();
        let score = req.answers.len() as i64;
        let create_tx = PollUserAnswer::new(
            poll.pk.clone(),
            poll_pk.clone(),
            req.answers,
            respondent,
            user,
        )
        .create_transact_write_item();

        let space_increment_tx = Poll::updater(&poll.pk, &poll.sk)
            .increase_user_response_count(1)
            .transact_write_item();

        transact_write!(&dynamo.client, create_tx, space_increment_tx)?;
        if let Ok(reward) = reward {
            UserReward::award(
                &dynamo.client,
                &biyard,
                reward,
                user_pk.clone(),
                Some(space.user_pk.clone()),
            )
            .await?;
        }

        if poll.is_default_poll() {
            SpaceDaoIncentiveScore::add_pre_score(
                &dynamo.client,
                &space_pk,
                &user_pk,
                score,
            )
            .await?;
        } else {
            SpaceDaoIncentiveScore::add_post_score(
                &dynamo.client,
                &space_pk,
                &user_pk,
                score,
            )
            .await?;
        }
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
