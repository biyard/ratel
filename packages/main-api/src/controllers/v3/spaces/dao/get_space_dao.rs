use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::SpaceDao;
use crate::types::{EntityType, SpacePartition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

pub async fn get_space_dao_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(SpacePathParam { space_pk }): SpacePath,
    NoApi(permissions): NoApi<Permissions>,
) -> Result<Json<SpaceDao>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let dao = SpaceDao::get(&dynamo.client, space_pk, Some(EntityType::SpaceDao)).await?;
    let dao = dao.ok_or(Error::NotFound("Space DAO not found".to_string()))?;

    Ok(Json(dao))
}
