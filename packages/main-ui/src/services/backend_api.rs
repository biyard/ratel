#![allow(non_snake_case)]
use by_macros::DioxusController;
use dto::*;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};

use crate::config;

#[derive(Clone, Copy, Default, DioxusController)]
pub struct BackendApi {
    pub endpoint: &'static str,
}

impl BackendApi {
    pub fn init() {
        let endpoint = config::get().main_api_endpoint;
        let srv = Self { endpoint };
        use_context_provider(move || srv);
    }

    pub async fn upload_metadata(
        &self,
        bytes: Vec<u8>,
        req: AssetPresignedUrisReadAction,
    ) -> Result<String> {
        let conf = config::get();
        let api_endpoint = conf.main_api_endpoint;

        let res = match AssetPresignedUris::get_client(api_endpoint)
            .get_presigned_uris(1, req.file_type.unwrap())
            .await
        {
            Ok(v) => Ok(v),
            Err(e) => {
                tracing::error!("Failed to upload metadata: network error {}", e);
                Err(ServerFnError::new(format!(
                    "upload metadata failed: network error: {:?}",
                    e
                )))
            }
        }
        .unwrap_or_default();

        let presigned_uri = res.presigned_uris[0].clone();
        let uri = res.uris[0].clone();

        tracing::debug!(
            "presigned_uri: {} uri: {}",
            presigned_uri.clone(),
            uri.clone()
        );

        let ext = match req.file_type {
            Some(v) => {
                if v == FileType::JPG {
                    "jpg"
                } else {
                    "png"
                }
            }
            None => "",
        };
        let content_type = HeaderValue::from_str(&format!("image/{}", ext)).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, content_type);

        match reqwest::Client::new()
            .put(presigned_uri.clone())
            .headers(headers)
            .body(bytes)
            .send()
            .await
        {
            Ok(_) => Ok(uri.clone()),
            Err(e) => {
                tracing::error!("Failed to upload metadata {:?}", e);

                Err(Error::UploadMetadataError(e.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountHint {
    pub address_hint: String,
    pub private_key_hint: String,
    pub restore_key: String,
    pub id: String,
    pub address: Option<String>,
}

impl AccountHint {
    pub fn seed(&self) -> Vec<u8> {
        hex::decode(&self.private_key_hint).unwrap()
    }

    pub fn is_registered(&self) -> bool {
        self.address.is_some()
    }
}
