use crate::{
    AppState, Error2,
    models::{
        space::{
            PollSpaceMetadata, PollSpacePathParam, PollSpaceResponse, PollSpaceSurveyResponse,
            SpaceCommon,
        },
        user::User,
    },
    types::{EntityType, Partition, TeamGroupPermission},
};

use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Path, State},
};

use aide::NoApi;
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct GetPollSpaceQueryParams {}

pub type GetPollSpaceResponse = PollSpaceResponse;

pub async fn get_poll_space_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(PollSpacePathParam {
        poll_space_pk: space_pk,
    }): Path<PollSpacePathParam>,
) -> Result<Json<GetPollSpaceResponse>, Error2> {
    // Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error2::NotFoundPollSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        user.as_ref().map(|u| &u.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let metadata = PollSpaceMetadata::query(&dynamo.client, &space_pk).await?;
    let mut poll_space_response = PollSpaceResponse::from(metadata);
    if let Some(user) = user {
        let my_survey_response = PollSpaceSurveyResponse::get(
            &dynamo.client,
            Partition::PollSpaceResponse(user.pk.to_string()),
            Some(EntityType::PollSpaceSurveyResponse(space_pk.to_string())),
        )
        .await?;

        poll_space_response.my_response = my_survey_response.map(|r| r.answers);
    }

    Ok(Json(poll_space_response))
}
