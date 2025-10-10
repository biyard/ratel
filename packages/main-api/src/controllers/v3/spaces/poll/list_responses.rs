use crate::models::space::{
    PollSpacePathParam, PollSpaceSurveyAnswerDto, PollSpaceSurveyResponse,
    PollSpaceSurveyResponseQueryOption, SpaceCommon,
};
use crate::models::user::User;
use crate::types::{EntityType, ListItemsResponse, TeamGroupPermission};
use crate::{AppState, Error2};

use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, Query, State};

use aide::NoApi;

use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ListResponsesQueryParams {
    bookmark: Option<String>,
    limit: Option<i32>,
}

pub async fn list_responses_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(PollSpacePathParam { poll_space_pk }): Path<PollSpacePathParam>,
    Query(ListResponsesQueryParams { bookmark, limit }): Query<ListResponsesQueryParams>,
) -> Result<Json<ListItemsResponse<PollSpaceSurveyAnswerDto>>, Error2> {
    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &poll_space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error2::NoPermission);
    }

    let mut option = PollSpaceSurveyResponseQueryOption::builder().limit(limit.unwrap_or(20));

    if bookmark.is_some() {
        option = option.bookmark(bookmark.unwrap());
    }

    let (responses, next_bookmark) = PollSpaceSurveyResponse::find_by_space_pk(
        &dynamo.client,
        &EntityType::PollSpaceSurveyResponse(poll_space_pk.to_string()),
        option,
    )
    .await?;
    let items = responses.into_iter().map(|e| e.into()).collect();
    Ok(Json(ListItemsResponse {
        items,
        bookmark: next_bookmark,
    }))
}
