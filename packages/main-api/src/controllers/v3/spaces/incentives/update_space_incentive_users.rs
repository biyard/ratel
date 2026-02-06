use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::{SpaceIncentive, SpaceIncentiveUser};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct UpdateSpaceIncentiveUsersRequest {
    pub incentive_sk: String,
    pub incentive_distributed: bool,
}

pub async fn update_space_incentive_users_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpdateSpaceIncentiveUsersRequest>,
) -> Result<Json<Vec<SpaceIncentiveUser>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    if req.incentive_sk.is_empty() {
        return Err(Error::BadRequest("incentive_sk is empty".to_string()));
    }

    let parsed_sk = req
        .incentive_sk
        .parse::<EntityType>()
        .map_err(|_| Error::BadRequest("invalid incentive sk".to_string()))?;

    let existing =
        SpaceIncentiveUser::get(&dynamo.client, space_pk.clone(), Some(parsed_sk.clone()))
            .await?
            .ok_or(Error::NotFound)
            .unwrap_or_default();
    let changed_count = if req.incentive_distributed {
        if existing.incentive_distributed { 0 } else { 1 }
    } else if existing.incentive_distributed {
        1
    } else {
        0
    } as i64;

    let item = SpaceIncentiveUser::updater(&space_pk, &parsed_sk)
        .with_incentive_distributed(req.incentive_distributed)
        .execute(&dynamo.client)
        .await?;

    if changed_count > 0 {
        let incentive = SpaceIncentive::get(
            &dynamo.client,
            space_pk.clone(),
            Some(EntityType::SpaceIncentive),
        )
        .await?
        .ok_or(Error::IncentiveNotFound)?;
        let delta = if req.incentive_distributed {
            -changed_count
        } else {
            changed_count
        };
        let remaining = (incentive.remaining_count + delta).max(0);
        SpaceIncentive::updater(space_pk.clone(), EntityType::SpaceIncentive)
            .with_remaining_count(remaining)
            .execute(&dynamo.client)
            .await?;
    }

    Ok(Json(vec![item]))
}
