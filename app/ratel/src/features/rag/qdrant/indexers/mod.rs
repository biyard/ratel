pub mod material_indexer;
pub mod post_indexer;
pub mod reply_indexer;

pub use material_indexer::*;
pub use post_indexer::*;
pub use reply_indexer::*;

use crate::common::{Error, Result};
use qdrant_client::qdrant::{
    CreateCollectionBuilder, DeletePointsBuilder, Distance, PointStruct, PointsIdsList,
    UpsertPointsBuilder, VectorParamsBuilder,
};
use std::collections::HashMap;
use std::sync::RwLock;

static ENSURED_COLLECTIONS: RwLock<Vec<String>> = RwLock::new(Vec::new());

/// Ensure a Qdrant collection exists, creating it with cosine/1024 if missing.
pub async fn ensure_collection(client: &qdrant_client::Qdrant, collection: &str) -> Result<()> {
    {
        let guard = ENSURED_COLLECTIONS.read().unwrap();
        if guard.iter().any(|c| c == collection) {
            return Ok(());
        }
    }

    if !client.collection_exists(collection).await.map_err(|e| {
        Error::InternalServerError(format!("Qdrant collection_exists failed: {e}"))
    })? {
        client
            .create_collection(
                CreateCollectionBuilder::new(collection)
                    .vectors_config(VectorParamsBuilder::new(1024, Distance::Cosine)),
            )
            .await
            .map_err(|e| {
                Error::InternalServerError(format!("Qdrant create_collection failed: {e}"))
            })?;
    }

    {
        let mut guard = ENSURED_COLLECTIONS.write().unwrap();
        if !guard.iter().any(|c| c == collection) {
            guard.push(collection.to_string());
        }
    }
    Ok(())
}

/// Upsert a single point into a Qdrant collection.
pub async fn upsert_point(
    client: &qdrant_client::Qdrant,
    collection: &str,
    point_id: &str,
    vector: Vec<f32>,
    payload: serde_json::Map<String, serde_json::Value>,
) -> Result<()> {
    ensure_collection(client, collection).await?;

    let qdrant_payload: HashMap<String, qdrant_client::qdrant::Value> = payload
        .into_iter()
        .map(|(k, v)| (k, json_to_qdrant_value(v)))
        .collect();

    let point = PointStruct::new(point_id.to_string(), vector, qdrant_payload);

    client
        .upsert_points(UpsertPointsBuilder::new(collection, vec![point]))
        .await
        .map_err(|e| Error::InternalServerError(format!("Qdrant upsert_points failed: {e}")))?;

    Ok(())
}

/// Delete a single point from a Qdrant collection by ID.
pub async fn delete_point(
    client: &qdrant_client::Qdrant,
    collection: &str,
    point_id: &str,
) -> Result<()> {
    ensure_collection(client, collection).await?;

    client
        .delete_points(
            DeletePointsBuilder::new(collection).points(PointsIdsList {
                ids: vec![point_id.to_string().into()],
            }),
        )
        .await
        .map_err(|e| Error::InternalServerError(format!("Qdrant delete_points failed: {e}")))?;

    Ok(())
}

/// Get collection name for global posts.
pub fn posts_collection() -> String {
    let cfg = crate::common::CommonConfig::default();
    format!("{}-posts", cfg.qdrant.prefix)
}

/// Get collection name for a discussion's AI moderator data.
pub fn discussion_collection(space_id: &str, discussion_sk: &str) -> String {
    let cfg = crate::common::CommonConfig::default();
    format!("{}-aimod-{}-{}", cfg.qdrant.prefix, space_id, discussion_sk)
}

fn json_to_qdrant_value(v: serde_json::Value) -> qdrant_client::qdrant::Value {
    match v {
        serde_json::Value::String(s) => qdrant_client::qdrant::Value::from(s),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                qdrant_client::qdrant::Value::from(i)
            } else if let Some(f) = n.as_f64() {
                qdrant_client::qdrant::Value::from(f)
            } else {
                qdrant_client::qdrant::Value::from(n.to_string())
            }
        }
        serde_json::Value::Bool(b) => qdrant_client::qdrant::Value::from(b),
        serde_json::Value::Null => qdrant_client::qdrant::Value::from(""),
        serde_json::Value::Array(arr) => {
            let list: Vec<qdrant_client::qdrant::Value> =
                arr.into_iter().map(json_to_qdrant_value).collect();
            qdrant_client::qdrant::Value::from(list)
        }
        serde_json::Value::Object(map) => {
            let inner: HashMap<String, qdrant_client::qdrant::Value> = map
                .into_iter()
                .map(|(k, v)| (k, json_to_qdrant_value(v)))
                .collect();
            qdrant_client::qdrant::Value {
                kind: Some(qdrant_client::qdrant::value::Kind::StructValue(
                    qdrant_client::qdrant::Struct { fields: inner },
                )),
            }
        }
    }
}
