use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::{SpaceIncentive, SpaceIncentiveUser};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error, User};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct GetSpaceIncentiveUserResponse {
    pub item: Option<SpaceIncentiveUser>,
    pub remaining_count: i64,
    pub total_count: i64,
}

pub async fn get_space_incentive_user_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<GetSpaceIncentiveUserResponse>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    let incentive_sk = EntityType::SpaceIncentiveUser(user.pk.to_string());
    let item =
        SpaceIncentiveUser::get(&dynamo.client, space_pk.clone(), Some(incentive_sk)).await?;
    let incentive = SpaceIncentive::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceIncentive),
    )
    .await?;
    let (remaining_count, total_count) = incentive
        .map(|item| (item.remaining_count, item.total_count))
        .unwrap_or((0, 0));

    Ok(Json(GetSpaceIncentiveUserResponse {
        item,
        remaining_count,
        total_count,
    }))
}
