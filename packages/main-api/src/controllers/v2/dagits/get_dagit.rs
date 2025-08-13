#![allow(unused)]
use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Artwork, Dagit, GroupPermission, Oracle, Result,
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
pub struct GetDagitPathParams {
    #[schemars(description = "Dagit ID")]
    pub dagit_id: i64,
}

pub async fn get_dagit_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(GetDagitPathParams { dagit_id }): Path<GetDagitPathParams>,
) -> Result<Json<Dagit>> {
    let dagit = Dagit::query_builder()
        .artworks_builder(Artwork::query_builder())
        .id_equals(dagit_id)
        .query()
        .map(Dagit::from)
        .fetch_one(&pool)
        .await?;
    check_perm(
        &pool,
        auth,
        dto::RatelResource::Space {
            space_id: dagit.space_id,
        },
        GroupPermission::ReadPosts,
    )
    .await?;

    Ok(Json(dagit))
}
