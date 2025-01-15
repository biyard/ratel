use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Default, PartialEq, Serialize, Deserialize)]
pub struct GetSignedUrlResponse {
    pub url: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetActionRequest {
    GetSignedUrl {
        filename: String,
        content_type: String,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetActionResponse {
    GetSignedUrl(String),
}
