use std::sync::atomic::{AtomicBool, Ordering};

use reqwest::Client;
use serde::Serialize;

use crate::common::{Error, Result};

static COLLECTION_ENSURED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct QdrantClient {
    url: String,
    collection_name: String,
    api_key: Option<String>,
    http: Client,
}

#[derive(Debug, Serialize)]
struct CreateCollectionBody {
    vectors: VectorsConfig,
}

#[derive(Debug, Serialize)]
struct VectorsConfig {
    size: u64,
    distance: &'static str,
}

#[derive(Debug, Serialize)]
struct UpsertPointsBody {
    points: Vec<PointBody>,
}

#[derive(Debug, Serialize)]
struct PointBody {
    id: String,
    vector: Vec<f32>,
    payload: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct DeletePointsBody {
    points: Vec<String>,
}

impl QdrantClient {
    pub fn new(url: String, collection_name: String, api_key: Option<String>) -> Self {
        Self {
            url,
            collection_name,
            api_key,
            http: Client::new(),
        }
    }

    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.url, path);
        let mut req = self.http.request(method, &url);
        if let Some(ref key) = self.api_key {
            req = req.header("api-key", key);
        }
        req
    }

    pub async fn ensure_collection(&self) -> Result<()> {
        if COLLECTION_ENSURED.load(Ordering::Relaxed) {
            return Ok(());
        }

        let resp = self
            .request(
                reqwest::Method::GET,
                &format!("/collections/{}", self.collection_name),
            )
            .send()
            .await
            .map_err(|e| {
                Error::InternalServerError(format!("Failed to check collection: {}", e))
            })?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND
            || resp.status() == reqwest::StatusCode::BAD_REQUEST
        {
            let body = CreateCollectionBody {
                vectors: VectorsConfig {
                    size: 1024,
                    distance: "Cosine",
                },
            };

            let create_resp = self
                .request(
                    reqwest::Method::PUT,
                    &format!("/collections/{}", self.collection_name),
                )
                .json(&body)
                .send()
                .await
                .map_err(|e| {
                    Error::InternalServerError(format!("Failed to create collection: {}", e))
                })?;

            if !create_resp.status().is_success() {
                let status = create_resp.status();
                let text = create_resp.text().await.unwrap_or_default();
                return Err(Error::InternalServerError(format!(
                    "Failed to create collection ({}): {}",
                    status, text
                )));
            }
        } else if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::InternalServerError(format!(
                "Failed to check collection ({}): {}",
                status, text
            )));
        }

        COLLECTION_ENSURED.store(true, Ordering::Relaxed);
        Ok(())
    }

    pub async fn upsert_point(
        &self,
        id: String,
        vector: Vec<f32>,
        payload: serde_json::Map<String, serde_json::Value>,
    ) -> Result<()> {
        self.ensure_collection().await?;

        let body = UpsertPointsBody {
            points: vec![PointBody {
                id,
                vector,
                payload,
            }],
        };

        let resp = self
            .request(
                reqwest::Method::PUT,
                &format!("/collections/{}/points", self.collection_name),
            )
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                Error::InternalServerError(format!("Failed to upsert point: {}", e))
            })?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::InternalServerError(format!(
                "Failed to upsert point ({}): {}",
                status, text
            )));
        }

        Ok(())
    }

    pub async fn delete_point(&self, id: String) -> Result<()> {
        self.ensure_collection().await?;

        let body = DeletePointsBody { points: vec![id] };

        let resp = self
            .request(
                reqwest::Method::POST,
                &format!("/collections/{}/points/delete", self.collection_name),
            )
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                Error::InternalServerError(format!("Failed to delete point: {}", e))
            })?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(Error::InternalServerError(format!(
                "Failed to delete point ({}): {}",
                status, text
            )));
        }

        Ok(())
    }
}
