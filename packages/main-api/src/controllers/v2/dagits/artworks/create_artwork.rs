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

    let file_json = serde_json::to_string(&req.file).map_err(|_| Error::BadRequest)?;

    let artwork_record: ArtworkRecord = rds_client
        .insert_returning(
            r#"
            WITH space_check AS (
                SELECT id, owner_id FROM spaces WHERE id = :space_id
            ),
            inserted_artwork AS (
                INSERT INTO artworks (owner_id, title, description, file)
                SELECT owner_id, :title, :description, :file::jsonb
                FROM space_check
                RETURNING id, owner_id
            ),
            inserted_detail AS (
                INSERT INTO artwork_details (artwork_id, owner_id, image)
                SELECT ia.id, ia.owner_id, :image
                FROM inserted_artwork ia
                RETURNING artwork_id
            ),
            inserted_dagit_artwork AS (
                INSERT INTO dagit_artworks (space_id, artwork_id)
                SELECT :space_id, ia.id
                FROM inserted_artwork ia
                RETURNING artwork_id
            )
            SELECT id FROM inserted_artwork
            "#,
            Some(vec![
                SqlParameter::builder()
                    .name("space_id")
                    .value(Field::LongValue(space_id))
                    .build(),
                SqlParameter::builder()
                    .name("title")
                    .value(Field::StringValue(req.title))
                    .build(),
                SqlParameter::builder()
                    .name("description")
                    .value(match req.description {
                        Some(desc) => Field::StringValue(desc),
                        None => Field::IsNull(true),
                    })
                    .build(),
                SqlParameter::builder()
                    .name("file")
                    .value(Field::StringValue(file_json))
                    .build(),
                SqlParameter::builder()
                    .name("image")
                    .value(Field::StringValue(url))
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
