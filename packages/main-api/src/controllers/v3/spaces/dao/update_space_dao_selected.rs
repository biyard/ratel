use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::{SpaceDao, SpaceDaoSelectedUser};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error};
use aide::NoApi;
use axum::extract::{Path, State};
use axum::Json;
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct UpdateSpaceDaoSelectedRequest {
    pub selected_sks: Vec<String>,
    pub reward_distributed: bool,
}

pub async fn update_space_dao_selected_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpdateSpaceDaoSelectedRequest>,
) -> Result<Json<Vec<SpaceDaoSelectedUser>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    if req.selected_sks.is_empty() {
        return Err(Error::BadRequest("selected_sks is empty".to_string()));
    }

    let mut parsed_sks = Vec::with_capacity(req.selected_sks.len());
    for sk in &req.selected_sks {
        let parsed = sk
            .parse::<EntityType>()
            .map_err(|_| Error::BadRequest("invalid selected sk".to_string()))?;
        parsed_sks.push(parsed);
    }

    let keys = parsed_sks
        .iter()
        .map(|sk| SpaceDaoSelectedUser::keys(&space_pk, sk))
        .collect::<Vec<_>>();
    let existing = SpaceDaoSelectedUser::batch_get(&dynamo.client, keys).await?;
    let changed_count = if req.reward_distributed {
        existing.iter().filter(|item| !item.reward_distributed).count()
    } else {
        existing.iter().filter(|item| item.reward_distributed).count()
    } as i64;

    let mut updated = Vec::with_capacity(parsed_sks.len());
    for sk in parsed_sks {
        let item = SpaceDaoSelectedUser::updater(&space_pk, &sk)
            .with_reward_distributed(req.reward_distributed)
            .execute(&dynamo.client)
            .await?;
        updated.push(item);
    }

    if changed_count > 0 {
        let dao = SpaceDao::get(
            &dynamo.client,
            space_pk.clone(),
            Some(EntityType::SpaceDao),
        )
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

    Ok(Json(updated))
}
