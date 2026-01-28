use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::SpaceDaoSampleUser;
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::{AppState, Error};
use aide::NoApi;
use axum::extract::{Path, State};
use axum::Json;
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct UpdateSpaceDaoSamplesRequest {
    pub sample_sks: Vec<String>,
    pub reward_distributed: bool,
}

pub async fn update_space_dao_samples_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<UpdateSpaceDaoSamplesRequest>,
) -> Result<Json<Vec<SpaceDaoSampleUser>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    if req.sample_sks.is_empty() {
        return Err(Error::BadRequest("sample_sks is empty".to_string()));
    }

    let mut updated = Vec::with_capacity(req.sample_sks.len());
    for sk in req.sample_sks {
        let sk = sk
            .parse::<EntityType>()
            .map_err(|_| Error::BadRequest("invalid sample sk".to_string()))?;

        let item = SpaceDaoSampleUser::updater(&space_pk, &sk)
            .with_reward_distributed(req.reward_distributed)
            .execute(&dynamo.client)
            .await?;
        updated.push(item);
    }

    Ok(Json(updated))
}
