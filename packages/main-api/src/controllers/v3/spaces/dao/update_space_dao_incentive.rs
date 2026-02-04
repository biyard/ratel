use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::{SpaceDao, SpaceDaoIncentiveUser};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct UpdateSpaceDaoIncentiveRequest {
    pub incentive_sk: String,
    pub incentive_distributed: bool,
}

pub async fn update_space_dao_incentive_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpdateSpaceDaoIncentiveRequest>,
) -> Result<Json<Vec<SpaceDaoIncentiveUser>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    if req.incentive_sk.is_empty() {
        return Err(Error::BadRequest("incentive_sk is empty".to_string()));
    }

    let parsed_sk = req
        .incentive_sk
        .parse::<EntityType>()
        .map_err(|_| Error::BadRequest("invalid incentive sk".to_string()))?;

    let existing =
        SpaceDaoIncentiveUser::get(&dynamo.client, space_pk.clone(), Some(parsed_sk.clone()))
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

    let item = SpaceDaoIncentiveUser::updater(&space_pk, &parsed_sk)
        .with_incentive_distributed(req.incentive_distributed)
        .execute(&dynamo.client)
        .await?;

    if changed_count > 0 {
        let dao = SpaceDao::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceDao))
            .await?
            .ok_or(Error::DaoNotFound)?;
        let delta = if req.incentive_distributed {
            -changed_count
        } else {
            changed_count
        };
        let remaining = (dao.remaining_count + delta).max(0);
        SpaceDao::updater(space_pk.clone(), EntityType::SpaceDao)
            .with_remaining_count(remaining)
            .execute(&dynamo.client)
            .await?;
    }

    Ok(Json(vec![item]))
}
