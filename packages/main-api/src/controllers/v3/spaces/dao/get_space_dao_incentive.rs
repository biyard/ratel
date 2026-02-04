use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::{SpaceDao, SpaceDaoIncentiveUser};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error, User};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct GetSpaceDaoIncentiveResponse {
    pub item: Option<SpaceDaoIncentiveUser>,
    pub remaining_count: i64,
    pub total_count: i64,
}

pub async fn get_space_dao_incentive_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<GetSpaceDaoIncentiveResponse>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let incentive_sk = EntityType::SpaceDaoIncentive(user.pk.to_string());
    let item =
        SpaceDaoIncentiveUser::get(&dynamo.client, space_pk.clone(), Some(incentive_sk)).await?;
    let dao = SpaceDao::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceDao)).await?;
    let (remaining_count, total_count) = dao
        .map(|item| (item.remaining_count, item.total_count))
        .unwrap_or((0, 0));

    Ok(Json(GetSpaceDaoIncentiveResponse {
        item,
        remaining_count,
        total_count,
    }))
}
