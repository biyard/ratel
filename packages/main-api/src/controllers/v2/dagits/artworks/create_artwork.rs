use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres, postgres::PgRow},
};
use std::sync::Arc;

use dto::*;

use crate::utils::sqs_client::SqsClient;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WatermarkTask {
    pub artwork_id: i64,
    pub original_url: String,
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

pub async fn create_artwork_handler(
    Extension(_auth): Extension<Option<Authorization>>,
    State((pool, sqs_client)): State<(Pool<Postgres>, Arc<SqsClient>)>,
    Path(CreateArtworkPathParams { space_id }): Path<CreateArtworkPathParams>,
    Json(req): Json<CreateArtworkRequest>,
) -> Result<Json<CreateArtworkResponse>> {
    let url = req.file.url.clone().ok_or(Error::BadRequest)?;

    let file_json = serde_json::to_string(&req.file).map_err(|_| Error::BadRequest)?;

    let query = r#"
            WITH space_check AS (
                SELECT id, owner_id FROM spaces WHERE id = $1
            ),
            inserted_artwork AS (
                INSERT INTO artworks (owner_id, title, description, file)
                SELECT owner_id, $2, $3, $4::jsonb
                FROM space_check
                RETURNING id, owner_id
            ),
            inserted_detail AS (
                INSERT INTO artwork_details (artwork_id, owner_id, image)
                SELECT ia.id, ia.owner_id, $5
                FROM inserted_artwork ia
                RETURNING artwork_id
            ),
            inserted_dagit_artwork AS (
                INSERT INTO dagit_artworks (space_id, artwork_id)
                SELECT $1, ia.id
                FROM inserted_artwork ia
                RETURNING artwork_id
            )
            SELECT id FROM inserted_artwork
            "#;

    let artwork_id = sqlx::query(query)
        .bind(space_id)
        .bind(req.title)
        .bind(req.description.as_deref())
        .bind(file_json)
        .bind(url.clone())
        .map(|row: PgRow| {
            use sqlx::Row;
            row.get::<i64, _>("id")
        })
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create artwork: {}", e);
            Error::ServerError("Failed to create artwork".to_string())
        })?;

    let task = WatermarkTask {
        artwork_id,
        original_url: url,
    };
    let message_body = serde_json::to_string(&task)
        .map_err(|_| Error::ServerError("Failed to serialize watermark task".to_string()))?;

    if let Err(e) = sqs_client.send_message(&message_body).await {
        tracing::error!("Failed to send watermark task to SQS: {}", e);
    }

    Ok(Json(CreateArtworkResponse { id: artwork_id }))
}
