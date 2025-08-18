#![allow(unused)]
use crate::utils::s3_upload::{self, PresignedUrl};
use crate::{config, security::check_perm};
use base64::{Engine, engine::general_purpose};
use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::ArtworkRepositoryUpdateRequest;
use dto::{
    Artwork, ArtworkDetail, Dagit, DagitArtwork, Error, File, GroupPermission, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
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
pub struct CreateArtworkPathParams {
    #[schemars(description = "Space ID")]
    pub space_id: i64,
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
    pub file: File,
}

pub async fn create_artwork_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(CreateArtworkPathParams { space_id }): Path<CreateArtworkPathParams>,
    Json(req): Json<CreateArtworkRequest>,
) -> Result<Json<Artwork>> {
    let url = req.file.url.clone().ok_or(Error::BadRequest)?;
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

    let mut tx = pool.begin().await?;
    let artwork = Artwork::get_repository(pool.clone())
        .insert_with_tx(
            &mut *tx,
            dagit.owner_id,
            req.title,
            req.description,
            req.file,
        )
        .await?
        .ok_or(dto::Error::ServerError(
            "Failed to create artwork".to_string(),
        ))?;

    ArtworkDetail::get_repository(pool.clone())
        .insert_with_tx(&mut *tx, artwork.id, dagit.owner_id, url.clone())
        .await?;

    DagitArtwork::get_repository(pool.clone())
        .insert_with_tx(&mut *tx, dagit.id, artwork.id)
        .await?;

    tx.commit().await?;

    // let pool_clone = pool.clone();
    // let artwork_id = artwork.id;
    // tokio::spawn(async move {
    //     if let Err(e) = process_watermark_async(pool_clone, artwork_id, url).await {
    //         tracing::error!(
    //             "Failed to process watermark for artwork {}: {}",
    //             artwork_id,
    //             e
    //         );
    //     }
    // });

    Ok(Json(artwork))
}

async fn process_watermark_async(
    pool: Pool<Postgres>,
    artwork_id: i64,
    original_url: String,
) -> Result<()> {
    let bytes = read_image_from_url(&original_url).await?;
    let watermarked_bytes =
        tokio::task::spawn_blocking(move || visible_watermarking(bytes)).await??;

    let config = config::get();
    let PresignedUrl {
        presigned_uris,
        uris,
        total_count: _,
    } = s3_upload::get_put_object_uri(&config.aws, &config.bucket, Some(1)).await?;

    let client = reqwest::Client::new();
    client
        .put(presigned_uris[0].clone())
        .body(watermarked_bytes)
        .send()
        .await?;

    Artwork::get_repository(pool.clone())
        .update(
            artwork_id,
            ArtworkRepositoryUpdateRequest {
                file: Some(File {
                    url: Some(uris[0].clone()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}

// pub async fn create_artwork_handler(
//     Extension(auth): Extension<Option<Authorization>>,
//     State(pool): State<Pool<Postgres>>,
//     Path(CreateArtworkPathParams { space_id }): Path<CreateArtworkPathParams>,
//     Json(mut req): Json<CreateArtworkRequest>,
// ) -> Result<Json<Artwork>> {
//     let url = req.file.url.clone().ok_or(Error::BadRequest)?;
//     let dagit = Dagit::query_builder(0)
//         .id_equals(space_id)
//         .query()
//         .map(Dagit::from)
//         .fetch_one(&pool)
//         .await?;

//     check_perm(
//         &pool,
//         auth,
//         dto::RatelResource::Space { space_id: dagit.id },
//         GroupPermission::ManageSpace,
//     )
//     .await?;

//     let bytes = read_image_from_url(&url).await?;
//     let bytes = visible_watermarking(bytes)?;
//     let config = config::get();
//     let PresignedUrl {
//         presigned_uris,
//         uris,
//         total_count: _,
//     } = s3_upload::get_put_object_uri(&config.aws, &config.bucket, Some(1)).await?;

//     let client = reqwest::Client::new();
//     client
//         .put(presigned_uris[0].clone())
//         .body(bytes)
//         .send()
//         .await?;

//     req.file.url = Some(uris[0].clone());

//     let mut tx = pool.begin().await?;
//     let artwork = Artwork::get_repository(pool.clone())
//         .insert_with_tx(
//             &mut *tx,
//             dagit.owner_id,
//             req.title,
//             req.description,
//             req.file,
//         )
//         .await?
//         .ok_or(dto::Error::ServerError(
//             "Failed to create artwork".to_string(),
//         ))?;

//     ArtworkDetail::get_repository(pool.clone())
//         .insert_with_tx(&mut *tx, artwork.id, dagit.owner_id, url)
//         .await?;

//     DagitArtwork::get_repository(pool.clone())
//         .insert_with_tx(&mut *tx, dagit.id, artwork.id)
//         .await?;
//     tx.commit().await?;
//     Ok(Json(artwork))
// }

async fn read_image_from_url(url: &str) -> Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let bytes = client.get(url).send().await?.bytes().await?;
    Ok(bytes.to_vec())
}

pub fn visible_watermarking(slice: Vec<u8>) -> Result<Vec<u8>> {
    let mut orig = photon_rs::PhotonImage::new_from_byteslice(slice);

    let width = orig.get_width();
    let height = orig.get_height();
    if width == 0 || height == 0 {
        tracing::error!("Image has zero width or height");
        return Err(Error::BadRequest);
    }
    // Update the path below to the correct location of your watermark image file
    let logo = include_bytes!("./protected.png");

    let logo = general_purpose::STANDARD.encode(logo.as_ref());

    let wm = photon_rs::PhotonImage::new_from_base64(&logo);

    let wm_width = wm.get_width();
    let wm_height = wm.get_height();

    let w_width = (width as f32 * 0.8) as u32;
    let w_height = ((w_width as f32 / wm_width as f32) * wm_height as f32) as u32;
    let wm = photon_rs::transform::resize(
        &wm,
        w_width,
        w_height,
        photon_rs::transform::SamplingFilter::Nearest,
    );
    let x = (width - w_width) / 2;
    let y = (height - w_height) / 2;

    photon_rs::multiple::watermark(&mut orig, &wm, x as i64, y as i64);
    let bytes = orig.get_bytes();

    Ok(bytes)
}
