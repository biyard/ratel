use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::SpaceDao;
use crate::types::{SpacePartition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceDaoRequest {
    pub contract_address: String,
    pub sampling_count: i64,
    pub reward_amount: i64,
}

pub async fn create_space_dao_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreateSpaceDaoRequest>,
) -> Result<Json<SpaceDao>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    if req.contract_address.trim().is_empty() {
        return Err(Error::BadRequest(
            "contract_address is required".to_string(),
        ));
    }
    if req.sampling_count <= 0 {
        return Err(Error::BadRequest(
            "sampling_count must be greater than 0".to_string(),
        ));
    }
    if req.reward_amount <= 0 {
        return Err(Error::BadRequest(
            "reward_amount must be greater than 0".to_string(),
        ));
    }

    let space_pk: SpacePartition = space_pk.into();
    let dao = SpaceDao::new(
        space_pk,
        req.contract_address,
        req.sampling_count,
        req.reward_amount,
    );

    dao.upsert(&dynamo.client).await?;

    Ok(Json(dao))
}
