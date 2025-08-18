use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Dagit, DagitOracle, Error, GroupPermission, Oracle, OracleType, RatelResource, Result,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};

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
pub struct CreateOracleRequest {
    #[schemars(description = "Dagit ID (optional)")]
    pub user_id: i64,

    #[schemars(description = "Dagit ID (optional)")]
    pub oracle_type: OracleType,

    #[schemars(description = "Space ID (optional)")]
    pub space_id: Option<i64>,
}

use crate::security::check_perm;

pub async fn create_oracle_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<CreateOracleRequest>,
) -> Result<Json<Oracle>> {
    //FIXME: Only Admin User can able to create oracle
    // check_perm(
    //     &pool,
    //     auth.clone(),
    //     RatelResource::Oracles,
    //     GroupPermission::ManageOracles,
    // )
    // .await?;
    let mut tx = pool.begin().await?;
    let oracle = Oracle::query_builder()
        .user_id_equals(req.user_id)
        .query()
        .fetch_optional(&mut *tx)
        .await?;
    if oracle.is_some() {
        return Err(Error::ServerError(
            "Oracle already exists for this user".to_string(),
        ));
    }
    let oracle = Oracle::get_repository(pool.clone())
        .insert_with_tx(&mut *tx, req.user_id, req.oracle_type)
        .await?
        .ok_or(Error::ServerError("Failed to create oracle".to_string()))?;
    if req.space_id.is_some() {
        let dagit = Dagit::query_builder(0)
            .id_equals(req.space_id.unwrap())
            .query()
            .map(Dagit::from)
            .fetch_one(&pool)
            .await?;
        check_perm(
            &pool,
            auth,
            RatelResource::Space { space_id: dagit.id },
            GroupPermission::ManageSpace,
        )
        .await?;
        DagitOracle::get_repository(pool.clone())
            .insert_with_tx(&mut *tx, req.space_id.unwrap(), oracle.id)
            .await?;
    }
    tx.commit().await?;
    Ok(Json(oracle))
}
