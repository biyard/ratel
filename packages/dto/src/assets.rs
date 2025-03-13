use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use bdk::prelude::*;

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
