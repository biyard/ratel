use std::sync::Arc;

use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
};

use crate::utils::rds_client::RdsClient;
use aws_sdk_rdsdata::types::{Field, SqlParameter};
use dto::*;

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
pub struct CreateArtworkResponse {
    #[schemars(description = "Artwork Id")]
    pub id: i64,
}
#[derive(serde::Deserialize, Debug)]
struct DagitRecord {
    id: i64,
    owner_id: i64,
}
#[derive(serde::Deserialize, Debug)]
struct ArtworkRecord {
    id: i64,
}

pub async fn create_artwork_handler(
    Extension(_auth): Extension<Option<Authorization>>,
    State((_pool, rds_client)): State<(Pool<Postgres>, Arc<RdsClient>)>,
    Path(CreateArtworkPathParams { space_id }): Path<CreateArtworkPathParams>,
    Json(req): Json<CreateArtworkRequest>,
) -> Result<Json<CreateArtworkResponse>> {
    let url = req.file.url.clone().ok_or(Error::BadRequest)?;

    let dagit: DagitRecord = rds_client
        .query_one(
            "SELECT id, owner_id FROM spaces WHERE id = :space_id",
            Some(vec![
                SqlParameter::builder()
                    .name("space_id")
                    .value(Field::LongValue(space_id))
                    .build(),
            ]),
        )
        .await?;
    let artwork_record: ArtworkRecord = rds_client
        .insert_returning(
            "INSERT INTO artworks (owner_id, title, description, file) 
         VALUES (:owner_id, :title, :description, :file::jsonb) 
         RETURNING id, owner_id, title, description, created_at, updated_at",
            Some(vec![
                SqlParameter::builder()
                    .name("owner_id")
                    .value(Field::LongValue(dagit.owner_id))
                    .build(),
                SqlParameter::builder()
                    .name("title")
                    .value(Field::StringValue(req.title.clone()))
                    .build(),
                SqlParameter::builder()
                    .name("description")
                    .value(match &req.description {
                        Some(desc) => Field::StringValue(desc.clone()),
                        None => Field::IsNull(true),
                    })
                    .build(),
                SqlParameter::builder()
                    .name("file")
                    .value(Field::StringValue(
                        serde_json::to_string(&req.file).map_err(|e| {
                            Error::ServerError(format!("Failed to serialize file: {}", e))
                        })?,
                    ))
                    .build(),
            ]),
        )
        .await?;

    rds_client
        .insert(
            "INSERT INTO artwork_details (artwork_id, owner_id, image)
         VALUES (:artwork_id, :owner_id, :image)",
            Some(vec![
                SqlParameter::builder()
                    .name("artwork_id")
                    .value(Field::LongValue(artwork_record.id))
                    .build(),
                SqlParameter::builder()
                    .name("owner_id")
                    .value(Field::LongValue(dagit.owner_id))
                    .build(),
                SqlParameter::builder()
                    .name("image")
                    .value(Field::StringValue(url.clone()))
                    .build(),
            ]),
        )
        .await?;

    rds_client
        .insert(
            "INSERT INTO dagit_artworks (space_id, artwork_id)
         VALUES (:space_id, :artwork_id)",
            Some(vec![
                SqlParameter::builder()
                    .name("space_id")
                    .value(Field::LongValue(dagit.id))
                    .build(),
                SqlParameter::builder()
                    .name("artwork_id")
                    .value(Field::LongValue(artwork_record.id))
                    .build(),
            ]),
        )
        .await?;

    Ok(Json(CreateArtworkResponse {
        id: artwork_record.id,
    }))
}

// async fn process_watermark_async(
//     pool: Pool<Postgres>,
//     artwork_id: i64,
//     original_url: String,
// ) -> Result<()> {
//     let bytes = read_image_from_url(&original_url).await?;
//     let watermarked_bytes =
//         tokio::task::spawn_blocking(move || visible_watermarking(bytes)).await??;

//     let config = config::get();
//     let PresignedUrl {
//         presigned_uris,
//         uris,
//         total_count: _,
//     } = s3_upload::get_put_object_uri(&config.aws, &config.bucket, Some(1)).await?;

//     let client = reqwest::Client::new();
//     client
//         .put(presigned_uris[0].clone())
//         .body(watermarked_bytes)
//         .send()
//         .await?;

//     Artwork::get_repository(pool.clone())
//         .update(
//             artwork_id,
//             ArtworkRepositoryUpdateRequest {
//                 file: Some(File {
//                     url: Some(uris[0].clone()),
//                     ..Default::default()
//                 }),
//                 ..Default::default()
//             },
//         )
//         .await?;

//     Ok(())
// }
