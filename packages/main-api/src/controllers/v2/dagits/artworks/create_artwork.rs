#![allow(unused)]
use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Artwork, ArtworkDetail, Dagit, DagitArtwork, File, GroupPermission, Result,
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
pub struct CreateArtworkPathParams {
    #[schemars(description = "Dagit ID")]
    pub dagit_id: i64,
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
pub struct CreateArtworkRequest {
    #[schemars(description = "Artwork title")]
    pub title: String,

    #[schemars(description = "Artwork description")]
    pub description: Option<String>,

    #[schemars(description = "Artwork file")]
    pub file: Vec<File>,
}

pub async fn create_artwork_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(CreateArtworkPathParams { dagit_id }): Path<CreateArtworkPathParams>,
    Json(req): Json<CreateArtworkRequest>,
) -> Result<Json<Artwork>> {
    let dagit = Dagit::query_builder()
        .id_equals(dagit_id)
        .query()
        .map(Dagit::from)
        .fetch_one(&pool)
        .await?;

    let user = check_perm(
        &pool,
        auth,
        dto::RatelResource::Space {
            space_id: dagit.space_id,
        },
        GroupPermission::ManageSpace,
    )
    .await?;

    let mut tx = pool.begin().await?;
    let artwork = Artwork::get_repository(pool.clone())
        .insert_with_tx(&mut *tx, user.id, req.title, req.description, req.file)
        .await?
        .ok_or(dto::Error::ServerError(
            "Failed to create artwork".to_string(),
        ))?;
    //FIXME: use file url
    ArtworkDetail::get_repository(pool.clone())
        .insert_with_tx(&mut *tx, artwork.id, user.id, "".to_string())
        .await?;

    DagitArtwork::get_repository(pool.clone())
        .insert_with_tx(&mut *tx, dagit.id, artwork.id)
        .await?;
    tx.commit().await?;
    Ok(Json(artwork))
}
