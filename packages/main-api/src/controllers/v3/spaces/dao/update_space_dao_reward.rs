use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::{SpaceDao, SpaceDaoRewardUser};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct UpdateSpaceDaoRewardRequest {
    pub reward_sk: String,
    pub reward_distributed: bool,
}

pub async fn update_space_dao_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpdateSpaceDaoRewardRequest>,
) -> Result<Json<Vec<SpaceDaoRewardUser>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceRead)?;

    if req.reward_sk.is_empty() {
        return Err(Error::BadRequest("reward_sk is empty".to_string()));
    }

    let parsed_sk = req
        .reward_sk
        .parse::<EntityType>()
        .map_err(|_| Error::BadRequest("invalid reward sk".to_string()))?;

    let existing =
        SpaceDaoRewardUser::get(&dynamo.client, space_pk.clone(), Some(parsed_sk.clone()))
            .await?
            .ok_or(Error::NotFound)
            .unwrap_or_default();
    let changed_count = if req.reward_distributed {
        if existing.reward_distributed { 0 } else { 1 }
    } else if existing.reward_distributed {
        1
    } else {
        0
    } as i64;

    let item = SpaceDaoRewardUser::updater(&space_pk, &parsed_sk)
        .with_reward_distributed(req.reward_distributed)
        .execute(&dynamo.client)
        .await?;

    if changed_count > 0 {
        let dao = SpaceDao::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceDao))
            .await?
            .ok_or(Error::DaoNotFound)?;
        let delta = if req.reward_distributed {
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
