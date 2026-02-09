use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::{SpaceIncentive, SpaceIncentiveCandidate};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::utils::space_incentive::collect_space_incentive_candidate_addresses;
use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpaceIncentiveCandidatesResponse {
    pub incentive_address: Option<String>,
    pub candidates: Vec<SpaceIncentiveCandidate>,
}

pub async fn list_space_incentive_candidates_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<ListSpaceIncentiveCandidatesResponse>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let incentive =
        SpaceIncentive::get(&dynamo.client, &space_pk, Some(EntityType::SpaceIncentive)).await?;
    let Some(incentive) = incentive else {
        return Ok(Json(ListSpaceIncentiveCandidatesResponse {
            incentive_address: None,
            candidates: Vec::new(),
        }));
    };

    let candidates =
        collect_space_incentive_candidate_addresses(&dynamo.client, &space_pk).await?;

    Ok(Json(ListSpaceIncentiveCandidatesResponse {
        incentive_address: Some(incentive.contract_address),
        candidates,
    }))
}
