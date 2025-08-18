use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Consensus, Dagit, GroupPermission, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
};

use crate::security::check_perm;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct CreateConsensusRequest {
    #[schemars(description = "Target Artwork ID for certification")]
    pub artwork_id: i64,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct CreateConsensusPathParams {
    #[schemars(description = "Space ID")]
    pub space_id: i64,
}

pub async fn create_consensus_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(CreateConsensusPathParams { space_id }): Path<CreateConsensusPathParams>,
    Json(req): Json<CreateConsensusRequest>,
) -> Result<Json<Consensus>> {
    check_perm(
        &pool,
        auth,
        dto::RatelResource::Space { space_id },
        GroupPermission::ManageSpace,
    )
    .await?;

    let dagit = Dagit::query_builder(0)
        .id_equals(space_id)
        .query()
        .map(Dagit::from)
        .fetch_one(&pool)
        .await?;

    let repo = Consensus::get_repository(pool.clone());

    let total_oracles = dagit.oracles.len() as i64;
    let res = repo
        .insert(dagit.id, req.artwork_id, total_oracles, None)
        .await?;

    Ok(Json(res))
}
