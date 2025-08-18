#![allow(unused)]
use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Artwork, Dagit, GroupPermission, Oracle, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
};
use tracing_subscriber::filter::combinator::Or;

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
pub struct GetDagitPathParams {
    #[schemars(description = "Space ID")]
    pub space_id: i64,
}

pub async fn get_dagit_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(GetDagitPathParams { space_id }): Path<GetDagitPathParams>,
) -> Result<Json<Dagit>> {
    tracing::debug!("get_dagit_handler called with space_id: {}", space_id);
    let user = extract_user(&pool, auth).await?;

    let oracle = Oracle::query_builder()
        .user_id_equals(user.id)
        .query()
        .map(Oracle::from)
        .fetch_one(&pool)
        .await;
    let oracle_id = match oracle {
        Ok(o) => o.id,
        Err(_) => 0,
    };
    let dagit = Dagit::query_builder(oracle_id)
        .artworks_builder(Artwork::query_builder(oracle_id))
        .id_equals(space_id)
        .query()
        .map(Dagit::from)
        .fetch_one(&pool)
        .await?;

    Ok(Json(dagit))
}
