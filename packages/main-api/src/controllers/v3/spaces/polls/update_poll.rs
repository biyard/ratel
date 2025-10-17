use crate::models::space::SpaceCommon;

use crate::controllers::v3::spaces::dto::*;

use crate::features::spaces::polls::{Poll, PollMetadata, PollResponse};
use crate::types::{
    EntityType, Partition, Question, SpacePublishState, SpaceStatus, TeamGroupPermission,
};
use crate::utils::time::get_now_timestamp_millis;
use crate::{AppState, Error};

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

pub async fn update_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpdatePollSpaceRequest>,
) -> crate::Result<Json<PollResponse>> {
    //Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundPoll);
    }

    // Check Permissions
    let (space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    // Only Draft or Published+Waiting state can be updated
    let is_updatable = match space_common.publish_state {
        SpacePublishState::Draft => true,
        SpacePublishState::Published => space_common.status == Some(SpaceStatus::Waiting),
        // _ => false,
    };

    if !is_updatable {
        return Err(Error::ImmutablePollState);
    }

    let poll_metadata = PollMetadata::query(&dynamo.client, &space_pk).await?;
    let mut response = PollResponse::from(poll_metadata);
    let now = get_now_timestamp_millis();
    response.updated_at = now;

    // Update existing survey
    let poll_updater = Poll::updater(&space_pk, &EntityType::SpacePoll);

    match req {
        UpdatePollSpaceRequest::Time {
            started_at,
            ended_at,
        } => {
            // Validate Time Range
            if started_at >= ended_at {
                return Err(Error::InvalidTimeRange);
            }
            poll_updater
                .with_updated_at(now)
                .with_started_at(started_at)
                .with_ended_at(ended_at)
                .execute(&dynamo.client)
                .await?;
            response.started_at = started_at;
            response.ended_at = ended_at;
            Ok(Json(response))
        }
        UpdatePollSpaceRequest::Question { ref questions } => {
            if questions.is_empty() {
                return Err(Error::PollInvalidQuestions);
            }
            Ok(Json(response))
        }
        UpdatePollSpaceRequest::ResponseEditable { response_editable } => {
            poll_updater
                .with_updated_at(now)
                .with_response_editable(response_editable)
                .execute(&dynamo.client)
                .await?;
            response.response_editable = response_editable;
            Ok(Json(response))
        }
    }

    // dynamo
    //     .client
    //     .transact_write_items()
    //     .set_transact_items(Some(vec![poll_survey_tx, space_tx, post_tx]))
    //     .send()
    //     .await
    //     .map_err(|e| {
    //         tracing::error!("Failed to update poll space: {:?}", e);
    //         Error2::InternalServerError("Failed to update poll space".into())
    //     })?;
}
