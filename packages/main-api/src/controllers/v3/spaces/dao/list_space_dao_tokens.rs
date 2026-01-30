use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::{SpaceDao, SpaceDaoToken};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error, ListItemsResponse};
use aide::NoApi;
use axum::extract::{Path, Query, State};
use axum::Json;
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpaceDaoTokensQuery {
    pub bookmark: Option<String>,
    pub limit: Option<i32>,
}

pub async fn list_space_dao_tokens_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListSpaceDaoTokensQuery { bookmark, limit }): Query<ListSpaceDaoTokensQuery>,
) -> Result<Json<ListItemsResponse<SpaceDaoToken>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let dao = SpaceDao::get(&dynamo.client, &space_pk, Some(EntityType::SpaceDao)).await?;
    let Some(dao) = dao else {
        return Ok(Json((Vec::<SpaceDaoToken>::new(), None).into()));
    };

    let mut opt = SpaceDaoToken::opt_with_bookmark(bookmark);
    if let Some(limit) = limit {
        opt = opt.limit(limit);
    }

    let (items, bookmark) =
        SpaceDaoToken::find_by_dao_address(&dynamo.client, &dao.contract_address, opt).await?;

    Ok(Json((items, bookmark).into()))
}
