use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::{SpaceIncentive, SpaceIncentiveToken};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error, ListItemsResponse};
use aide::NoApi;
use axum::extract::{Path, Query, State};
use axum::Json;
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpaceIncentiveTokensQuery {
    pub bookmark: Option<String>,
    pub limit: Option<i32>,
}

pub async fn list_space_incentive_tokens_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListSpaceIncentiveTokensQuery { bookmark, limit }): Query<ListSpaceIncentiveTokensQuery>,
) -> Result<Json<ListItemsResponse<SpaceIncentiveToken>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let incentive =
        SpaceIncentive::get(&dynamo.client, &space_pk, Some(EntityType::SpaceIncentive)).await?;
    let Some(incentive) = incentive else {
        return Ok(Json((Vec::<SpaceIncentiveToken>::new(), None).into()));
    };

    let mut opt = SpaceIncentiveToken::opt_with_bookmark(bookmark);
    if let Some(limit) = limit {
        opt = opt.limit(limit);
    }

    let (items, bookmark) = SpaceIncentiveToken::find_by_incentive_address(
        &dynamo.client,
        &incentive.contract_address,
        opt,
    )
    .await?;

    Ok(Json((items, bookmark).into()))
}
