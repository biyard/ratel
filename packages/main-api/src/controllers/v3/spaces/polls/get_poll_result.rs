use crate::features::spaces::polls::{PollPathParam, PollResultResponse, PollUserAnswer};
use crate::models::space::SpaceCommon;
use crate::models::user::User;

use crate::controllers::v3::spaces::dto::*;
use crate::types::{EntityType, Partition, TeamGroupPermission};
use crate::utils::time::get_now_timestamp_millis;
use crate::{AppState, Error};

use crate::features::spaces::polls::PollPath;
use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, State};

use aide::NoApi;

pub async fn get_poll_result(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
) -> Result<Json<PollResultResponse>, Error> {
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

    let poll_pk = match poll_sk {
        EntityType::SpacePoll(v) => Partition::Poll(v.to_string()),
        _ => Partition::Poll("".to_string()),
    };
    //FIXME: This logic should be removed once the fetcher is ready.
    // This logic is extremely computationally intensive.
    // This needs to be changed to perform a summary at the end of the call or at specific intervals and store the results.
    // Currently, the summary is always recalculated from the response.
    let (summaries, summaries_by_gender, summaries_by_age, summaries_by_school) =
        PollUserAnswer::summarize_responses_with_attribute(&dynamo.client, &space_pk, &poll_pk)
            .await?;

    Ok(Json(PollResultResponse {
        created_at: get_now_timestamp_millis(),
        summaries,
        summaries_by_age,
        summaries_by_gender,
        summaries_by_school,
    }))

    // let res = PollSpaceSurveyResult::get(
    //     &dynamo.client,
    //     &poll_space_pk,
    //     Some(EntityType::PollSpaceSurveyResult),
    // )
    // .await?
    // .ok_or(Error::NotFoundSurveySummary)?;

    // Ok(Json(PollSpaceSurveySummary {
    //     created_at: res.created_at,
    //     summaries: res.summaries,
    // }))
}
