use crate::{
    AppState, Error,
    models::{TeamGroup, TeamGroupQueryOption},
    types::{
        EntityType, Pagination, PaginationQuery, Partition, TeamPartition,
        list_items_response::ListItemsResponse,
    },
};
use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;

use crate::controllers::v3::teams::dto::TeamGroupResponse;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ListGroupsPathParams {
    pub team_pk: TeamPartition,
}

pub async fn list_groups_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(ListGroupsPathParams { team_pk }): Path<ListGroupsPathParams>,
    Query(Pagination { bookmark }): PaginationQuery,
) -> Result<Json<ListItemsResponse<TeamGroupResponse>>, Error> {
    // Set up pagination
    let mut query_options = TeamGroupQueryOption::builder()
        .limit(50)
        .sk(EntityType::TeamGroup(String::default()).to_string());

    if let Some(bookmark_str) = bookmark {
        query_options = query_options.bookmark(bookmark_str);
    }

    // Query all TeamGroup entities for this team with pagination
    let (groups, next_bookmark) = TeamGroup::query(&dynamo.client, team_pk, query_options).await?;

    // Convert to response format
    let group_responses: Vec<TeamGroupResponse> =
        groups.into_iter().map(TeamGroupResponse::from).collect();

    Ok(Json(ListItemsResponse {
        items: group_responses,
        bookmark: next_bookmark,
    }))
}
