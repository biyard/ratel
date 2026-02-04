use crate::features::spaces::files::FileLinkTarget;
use bdk::prelude::*;

/// Query parameter for getting files by target
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct GetFilesByTargetRequest {
    #[schemars(description = "Target location to query files for")]
    pub target: FileLinkTarget,
}

/// Response with files for a specific target
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct GetFilesByTargetResponse {
    pub target: FileLinkTarget,
    pub file_urls: Vec<String>,
}

/// Response for listing all file links in a space
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ListFileLinksResponse {
    pub file_links: Vec<FileLinkInfo>,
}

/// Information about a file and its links
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct FileLinkInfo {
    pub file_url: String,
    pub link_target: FileLinkTarget,
    pub created_at: i64,
    pub updated_at: i64,
}
