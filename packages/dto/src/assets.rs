use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[derive(Debug, Clone, Eq, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct GetSignedUrlResponse {
    pub url: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum AssetActionRequest {
    GetSignedUrl {
        filename: String,
        content_type: String,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum AssetActionResponse {
    GetSignedUrl(String),
}
