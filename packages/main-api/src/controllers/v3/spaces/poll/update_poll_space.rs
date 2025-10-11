use crate::models::feed::Post;
use crate::models::space::{
    PollSpaceMetadata, PollSpacePathParam, PollSpaceResponse, PollSpaceSurvey, SpaceCommon,
    TimeRange,
};
use crate::types::{
    EntityType, Partition, SpacePublishState, SpaceStatus, SurveyQuestion, TeamGroupPermission,
};
use crate::{AppState, Error2};

use bdk::prelude::*;

use by_axum::axum::extract::{Json, Path, State};

use crate::models::user::User;
use aide::NoApi;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpdatePollSpaceRequest {
    pub title: String,
    pub html_content: String,
    pub time_range: TimeRange,
    pub questions: Vec<SurveyQuestion>,
}

pub type UpdatePollSpaceResponse = PollSpaceResponse;

pub async fn update_poll_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(PollSpacePathParam { poll_space_pk }): Path<PollSpacePathParam>,
    Json(UpdatePollSpaceRequest {
        title,
        html_content,
        questions,
        time_range,
    }): Json<UpdatePollSpaceRequest>,
) -> Result<Json<UpdatePollSpaceResponse>, Error2> {
    //Request Validation
    if !matches!(poll_space_pk, Partition::PollSpace(_)) {
        return Err(Error2::NotFoundPollSpace);
    }

    // Check Permissions
    let (space, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &poll_space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }
    if time_range.is_valid() {
        return Err(Error2::InvalidTimeRange);
    }

    // Check Space Existence
    let space_common = SpaceCommon::get(
        &dynamo.client,
        &poll_space_pk,
        Some(crate::types::EntityType::SpaceCommon),
    )
    .await?
    .ok_or(Error2::SpaceNotFound)?;

    // Only Draft or Published+Waiting state can be updated
    let is_updatable = match space_common.publish_state {
        SpacePublishState::Draft => true,
        SpacePublishState::Published => space_common.status == Some(SpaceStatus::Waiting),
        // _ => false,
    };

    if !is_updatable {
        return Err(Error2::ImmutablePollSpaceState);
    }

    // Update Poll Space

    let poll_space_tx =
        PollSpaceSurvey::new(poll_space_pk.clone(), questions).create_transact_write_item();

    let space_tx = SpaceCommon::updater(&poll_space_pk, &space_common.sk)
        .with_started_at(time_range.0)
        .with_ended_at(time_range.1)
        .transact_write_item();

    let post_tx = Post::updater(&space.post_pk, &EntityType::Post)
        .with_title(title)
        .with_html_contents(html_content)
        .transact_write_item();

    dynamo
        .client
        .transact_write_items()
        .set_transact_items(Some(vec![poll_space_tx, space_tx, post_tx]))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to update poll space: {}", e);
            Error2::InternalServerError("Failed to update poll space".into())
        })?;

    let poll_metadata = PollSpaceMetadata::query(&dynamo.client, &poll_space_pk).await?;
    let response = PollSpaceResponse::from(poll_metadata);
    Ok(Json(response))
}
