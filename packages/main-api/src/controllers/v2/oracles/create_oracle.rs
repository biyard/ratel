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

    #[schemars(description = "Dagit ID (optional)")]
    pub dagit_id: Option<i64>,
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

    let oracle = Oracle::get_repository(pool.clone())
        .insert_with_tx(&mut *tx, req.user_id, req.oracle_type)
        .await?
        .ok_or(Error::ServerError("Failed to create oracle".to_string()))?;
    if req.dagit_id.is_some() {
        let dagit = Dagit::query_builder()
            .id_equals(req.dagit_id.unwrap())
            .query()
            .map(Dagit::from)
            .fetch_one(&pool)
            .await?;
        check_perm(
            &pool,
            auth,
            RatelResource::Space {
                space_id: dagit.space_id,
            },
            GroupPermission::ManageSpace,
        )
        .await?;
        DagitOracle::get_repository(pool.clone())
            .insert_with_tx(&mut *tx, req.dagit_id.unwrap(), oracle.id)
            .await?;
    }
    tx.commit().await?;
    Ok(Json(oracle))
}
