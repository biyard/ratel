use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::{SpaceDao, SpaceDaoCandidate};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::utils::space_dao_reward::collect_space_dao_candidate_addresses;
use crate::{AppState, Error};
use aide::NoApi;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct ListSpaceDaoCandidatesResponse {
    pub dao_address: Option<String>,
    pub candidates: Vec<SpaceDaoCandidate>,
}

pub async fn list_space_dao_candidates_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): SpacePath,
) -> Result<Json<ListSpaceDaoCandidatesResponse>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    let dao = SpaceDao::get(&dynamo.client, &space_pk, Some(EntityType::SpaceDao)).await?;
    let Some(dao) = dao else {
        return Ok(Json(ListSpaceDaoCandidatesResponse {
            dao_address: None,
            candidates: Vec::new(),
        }));
    };

    let candidates =
        collect_space_dao_candidate_addresses(&dynamo.client, &space_pk).await?;

    Ok(Json(ListSpaceDaoCandidatesResponse {
        dao_address: Some(dao.contract_address),
        candidates,
    }))
}
