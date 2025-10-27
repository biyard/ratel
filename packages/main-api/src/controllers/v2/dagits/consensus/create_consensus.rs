use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Consensus, GroupPermission, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres, postgres::PgRow},
};

use crate::{security::check_perm, utils::users::extract_user};

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
    let user = extract_user(&pool, auth.clone()).await.unwrap_or_default();

    check_perm(
        &pool,
        auth,
        dto::RatelResource::Space {
            team_id: user.id,
            space_id,
        },
        GroupPermission::ManageSpace,
    )
    .await?;

    let total_oracles = sqlx::query("SELECT COUNT(*) FROM dagit_oracles WHERE space_id = $1")
        .bind(space_id)
        .map(|row: PgRow| {
            use sqlx::Row;
            row.get::<i64, _>(0)
        })
        .fetch_one(&pool)
        .await?;
    let repo = Consensus::get_repository(pool.clone());

    let res = repo
        .insert(space_id, req.artwork_id, total_oracles, None)
        .await?;

    Ok(Json(res))
}
