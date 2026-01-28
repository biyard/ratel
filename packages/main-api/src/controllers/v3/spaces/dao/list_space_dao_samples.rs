use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::SpaceDaoSampleUser;
use crate::types::{Permissions, TeamGroupPermission};
use crate::{AppState, Error, ListItemsResponse};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, Query, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpaceDaoSamplesQuery {
    pub bookmark: Option<String>,
    pub limit: Option<i32>,
}

pub async fn list_space_dao_samples_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Query(ListSpaceDaoSamplesQuery { bookmark, limit }): Query<ListSpaceDaoSamplesQuery>,
) -> Result<Json<ListItemsResponse<SpaceDaoSampleUser>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let mut opt = SpaceDaoSampleUser::opt_with_bookmark(bookmark);
    if let Some(limit) = limit {
        opt = opt.limit(limit);
    }

    let (items, bookmark) =
        SpaceDaoSampleUser::find_by_space(&dynamo.client, &space_pk, opt).await?;

    Ok(Json((items, bookmark).into()))
}
