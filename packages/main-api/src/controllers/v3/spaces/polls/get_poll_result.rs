use crate::models::space::SpaceCommon;
use crate::models::user::User;

use crate::controllers::v3::spaces::dto::*;
use crate::models::PollUserResponse;
use crate::types::TeamGroupPermission;
use crate::utils::time::get_now_timestamp_millis;
use crate::{AppState, Error2};

use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, State};

use super::dto::PollResultResponse;
use aide::NoApi;

pub async fn get_poll_result(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<PollResultResponse>, Error2> {
    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }
    //FIXME: This logic should be removed once the fetcher is ready.
    // This logic is extremely computationally intensive.
    // This needs to be changed to perform a summary at the end of the call or at specific intervals and store the results.
    // Currently, the summary is always recalculated from the response.
    let summaries = PollUserResponse::summarize_responses(&dynamo.client, &space_pk).await?;

    Ok(Json(PollResultResponse {
        created_at: get_now_timestamp_millis(),
        summaries,
    }))

    // let res = PollSpaceSurveyResult::get(
    //     &dynamo.client,
    //     &poll_space_pk,
    //     Some(EntityType::PollSpaceSurveyResult),
    // )
    // .await?
    // .ok_or(Error2::NotFoundSurveySummary)?;

    // Ok(Json(PollSpaceSurveySummary {
    //     created_at: res.created_at,
    //     summaries: res.summaries,
    // }))
}
