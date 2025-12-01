use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::rewards::{SpaceReward, SpaceRewardResponse};
use crate::models::space::SpaceCommon;
use crate::types::{EntityType, ListItemsQuery, ListItemsResponse, Pagination, SpacePublishState};
use crate::{
    AppState, Error, Permissions, Result,
    models::user::User,
    types::{Partition, TeamGroupPermission},
};

use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Extension, Path, Query, State},
};

use aide::NoApi;

pub async fn list_rewards_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(Pagination { bookmark }): ListItemsQuery,
) -> Result<Json<ListItemsResponse<SpaceRewardResponse>>> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let (reward_response, bookmark) =
        SpaceReward::list_by_space(&dynamo.client, &space_pk, Some(50), bookmark).await?;

    Ok(Json(ListItemsResponse {
        items: reward_response
            .into_iter()
            .map(SpaceRewardResponse::from)
            .collect(),
        bookmark,
    }))
}
