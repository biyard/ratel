use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::SpaceIncentive;
use crate::types::{SpacePartition, TeamGroupPermission};
use crate::{AppState, Error, Permissions};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceIncentiveRequest {
    pub contract_address: String,
    pub deploy_block: i64,
}

pub async fn create_space_incentive_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<CreateSpaceIncentiveRequest>,
) -> Result<Json<SpaceIncentive>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    if req.contract_address.trim().is_empty() {
        return Err(Error::BadRequest(
            "contract_address is required".to_string(),
        ));
    }
    if req.deploy_block < 0 {
        return Err(Error::BadRequest(
            "deploy_block must be 0 or greater".to_string(),
        ));
    }

    let space_pk: SpacePartition = space_pk.into();
    let incentive = SpaceIncentive::new(
        space_pk,
        req.contract_address,
        req.deploy_block,
    );

    incentive.upsert(&dynamo.client).await?;

    Ok(Json(incentive))
}
