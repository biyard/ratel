use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Dagit, DagitOracle, GroupPermission, Result,
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
pub struct AddOracleRequest {
    #[schemars(description = "Oracle ID")]
    pub oracle_id: i64,
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
pub struct AddOraclePathParams {
    #[schemars(description = "Space ID")]
    pub space_id: i64,
}
pub async fn add_oracle_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(AddOraclePathParams { space_id }): Path<AddOraclePathParams>,
    Json(req): Json<AddOracleRequest>,
) -> Result<Json<Dagit>> {
    let dagit = Dagit::query_builder(0)
        .id_equals(space_id)
        .query()
        .map(Dagit::from)
        .fetch_one(&pool)
        .await?;
    check_perm(
        &pool,
        auth,
        dto::RatelResource::Space { space_id: dagit.id },
        GroupPermission::ManageSpace,
    )
    .await?;
    let dagit_oracle = DagitOracle::get_repository(pool.clone())
        .insert(space_id, req.oracle_id)
        .await?;

    let dagit = Dagit::query_builder(dagit_oracle.oracle_id)
        .id_equals(space_id)
        .query()
        .map(Dagit::from)
        .fetch_one(&pool)
        .await?;

    Ok(Json(dagit))
}
