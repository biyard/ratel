use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::SpaceIncentive;
use crate::types::{EntityType, SpacePartition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

pub async fn get_space_incentive_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Path(SpacePathParam { space_pk }): SpacePath,
    NoApi(permissions): NoApi<Permissions>,
) -> Result<Json<SpaceIncentive>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let incentive =
        SpaceIncentive::get(&dynamo.client, space_pk, Some(EntityType::SpaceIncentive)).await?;
    let incentive =
        incentive.ok_or(Error::NotFound("Space incentive not found".to_string()))?;

    Ok(Json(incentive))
}
