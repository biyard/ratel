use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    DagitOracle, DagitWithoutJoin, GroupPermission, Oracle, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
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
pub struct AddOracleRequest {
    #[schemars(description = "User ID")]
    pub user_id: i64,
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
) -> Result<()> {
    tracing::info!("TIMER 1: {:?}", chrono::Utc::now());
    let user = extract_user(&pool, auth.clone()).await.unwrap_or_default();

    let _dagit = DagitWithoutJoin::query_builder()
        .id_equals(space_id)
        .query()
        .map(DagitWithoutJoin::from)
        .fetch_one(&pool)
        .await?;

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
    let oracle_id = Oracle::query_builder()
        .user_id_equals(req.user_id)
        .query()
        .map(Oracle::from)
        .fetch_one(&pool)
        .await?
        .id;
    DagitOracle::get_repository(pool.clone())
        .insert(space_id, oracle_id)
        .await?;

    Ok(())
}
