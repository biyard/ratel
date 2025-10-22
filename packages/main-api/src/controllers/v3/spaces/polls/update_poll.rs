use crate::models::space::SpaceCommon;

use crate::features::spaces::polls::*;
use crate::types::{EntityType, Partition, Question, TeamGroupPermission};
use crate::utils::time::get_now_timestamp_millis;
use crate::{AppState, Error, transact_write};

use bdk::prelude::*;

use by_axum::axum::extract::{Json, Path, State};

use crate::models::user::User;
use aide::NoApi;
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum UpdatePollSpaceRequest {
    Time { started_at: i64, ended_at: i64 },
    Question { questions: Vec<Question> },
    ResponseEditable { response_editable: bool },
}

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Default)]
pub struct UpdatePollSpaceResponse {
    pub status: String,
}

pub async fn update_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
    Json(req): Json<UpdatePollSpaceRequest>,
) -> crate::Result<Json<UpdatePollSpaceResponse>> {
    //Request Validation
    if !matches!(space_pk, Partition::Space(_)) || !matches!(poll_sk, EntityType::SpacePoll(_)) {
        return Err(Error::NotFoundPoll);
    }

    // Check Permissions
    let (_space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let now = get_now_timestamp_millis();

    let space = SpaceCommon::updater(&space_pk, EntityType::SpaceCommon).with_updated_at(now);
    let mut poll_updater = Poll::updater(&space_pk, &poll_sk).with_updated_at(now);

    match req {
        UpdatePollSpaceRequest::Time {
            started_at,
            ended_at,
        } => {
            // Validate Time Range
            if started_at >= ended_at {
                return Err(Error::InvalidTimeRange);
            }
            poll_updater = poll_updater
                .with_started_at(started_at)
                .with_ended_at(ended_at);
        }
        UpdatePollSpaceRequest::Question { questions } => {
            if questions.is_empty() {
                return Err(Error::PollInvalidQuestions);
            }
            poll_updater = poll_updater.with_questions(questions.clone());

            // Also create/update PollQuestion entity for result aggregation
            let poll_question = PollQuestion::new(space_pk.clone(), questions);
            poll_question.create(&dynamo.client).await?;
        }
        UpdatePollSpaceRequest::ResponseEditable { response_editable } => {
            poll_updater = poll_updater.with_response_editable(response_editable);
        }
    }

    transact_write!(
        &dynamo.client,
        space.transact_write_item(),
        poll_updater.transact_write_item(),
    )?;

    Ok(Json(UpdatePollSpaceResponse {
        status: "success".to_string(),
    }))
}
