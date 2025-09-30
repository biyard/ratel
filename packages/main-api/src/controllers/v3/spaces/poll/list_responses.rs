use crate::models::space::{PollSpaceSurveyResponse, PollSpaceSurveyResponseQueryOption};
use crate::types::{EntityType, Partition};
use crate::utils::dynamo_extractor::extract_user_from_session;
use crate::{AppState, Error2};
use dto::by_axum::axum::{
    Extension,
    extract::{Json, Path, Query, State},
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ListResponsesPathParams {
    #[serde(deserialize_with = "crate::types::path_param_string_to_partition")]
    poll_space_pk: Partition,
}

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ListResponsesQueryParams {
    bookmark: Option<String>,
    limit: Option<i32>,
}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct ListSurveyResponse {
    items: Vec<PollSpaceSurveyResponse>,
    bookmark: Option<String>,
}

pub async fn list_responses_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<tower_sessions::Session>,
    Path(ListResponsesPathParams { poll_space_pk }): Path<ListResponsesPathParams>,
    Query(ListResponsesQueryParams { bookmark, limit }): Query<ListResponsesQueryParams>,
) -> Result<Json<ListSurveyResponse>, Error2> {
    let _user = extract_user_from_session(&dynamo.client, &session).await;
    // FIXME: Need to check if the user has permission to view the responses

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

    Ok(Json(ListSurveyResponse {
        items: responses,
        bookmark: next_bookmark,
    }))
}
