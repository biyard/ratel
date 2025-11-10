use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::polls::{Poll, PollUserAnswer};
use crate::models::user::User;
use crate::models::SpaceCommon;
use crate::types::{EntityType, Partition, SpaceType};
use crate::{AppState, Error};
use aide::NoApi;
use axum::extract::{Path, State};
use axum::Json;
use bdk::prelude::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo)]
pub struct CheckPrerequisitesResponse {
    /// Whether all pre-requisites are completed
    pub completed: bool,
    /// Type of pre-requisite (e.g., "default_poll", "onboarding")
    pub prerequisite_type: Option<String>,
    /// The poll PK if the pre-requisite is a poll
    pub poll_pk: Option<String>,
    /// Message describing what needs to be completed
    pub message: Option<String>,
}

/// Check if user has completed all pre-requisites for a space
pub async fn check_prerequisites_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<CheckPrerequisitesResponse>, Error> {
    // If user is not logged in, no prerequisites check needed
    let Some(user) = user else {
        return Ok(Json(CheckPrerequisitesResponse {
            completed: true,
            prerequisite_type: None,
            poll_pk: None,
            message: None,
        }));
    };

    // Get space common to check space type
    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(EntityType::SpaceCommon))
        .await?
        .ok_or(Error::NotFoundSpace)?;

    // Check pre-requisites based on space type
    match space_common.space_type {
        SpaceType::Deliberation => {
            check_deliberation_prerequisites(&dynamo.client, &space_pk, &user.pk).await
        }
        // Other space types can add their own pre-requisites here
        _ => Ok(Json(CheckPrerequisitesResponse {
            completed: true,
            prerequisite_type: None,
            poll_pk: None,
            message: None,
        })),
    }
}

/// Check if user has completed pre-poll survey for Deliberation space
async fn check_deliberation_prerequisites(
    client: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    user_pk: &Partition,
) -> Result<Json<CheckPrerequisitesResponse>, Error> {
    // Get the default poll (pre-poll survey) for this space
    let space_id = match space_pk {
        Partition::Space(id) => id.clone(),
        _ => return Err(Error::NotFoundSpace),
    };

    let default_poll_sk = EntityType::SpacePoll(space_id.clone());
    let default_poll = Poll::get(client, space_pk, Some(default_poll_sk.clone())).await?;

    // If no default poll exists, no prerequisite
    let Some(poll) = default_poll else {
        return Ok(Json(CheckPrerequisitesResponse {
            completed: true,
            prerequisite_type: None,
            poll_pk: None,
            message: None,
        }));
    };

    // Check if poll has questions - if not, no prerequisite
    if poll.questions.is_empty() {
        return Ok(Json(CheckPrerequisitesResponse {
            completed: true,
            prerequisite_type: None,
            poll_pk: None,
            message: None,
        }));
    }

    // Check if user has answered the default poll
    let poll_pk = Partition::Poll(space_id.clone());
    let user_answer = PollUserAnswer::find_one(client, space_pk, &poll_pk, user_pk).await?;

    if user_answer.is_some() {
        // User has completed the pre-poll survey
        Ok(Json(CheckPrerequisitesResponse {
            completed: true,
            prerequisite_type: Some("default_poll".to_string()),
            poll_pk: Some(poll_pk.to_string()),
            message: None,
        }))
    } else {
        // User needs to complete the pre-poll survey
        Ok(Json(CheckPrerequisitesResponse {
            completed: false,
            prerequisite_type: Some("default_poll".to_string()),
            poll_pk: Some(poll_pk.to_string()),
            message: Some("Please complete the pre-poll survey before accessing the space".to_string()),
        }))
    }
}
