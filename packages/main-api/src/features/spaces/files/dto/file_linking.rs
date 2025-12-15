use crate::features::spaces::files::FileLinkTarget;
use bdk::prelude::*;

/// Request to link a file to additional targets
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct LinkFileRequest {
    #[schemars(description = "File URL to link")]
    pub file_url: String,

    #[schemars(description = "Target locations to link the file to")]
    pub targets: Vec<FileLinkTarget>,
}

/// Response after linking a file
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct LinkFileResponse {
    pub file_url: String,
    pub linked_targets: Vec<FileLinkTarget>,
}

/// Request to unlink a file from targets
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UnlinkFileRequest {
    #[schemars(description = "File URL to unlink")]
    pub file_url: String,

    #[schemars(description = "Targets to unlink from")]
    pub targets: Vec<FileLinkTarget>,
}

/// Response after unlinking a file
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct UnlinkFileResponse {
    pub file_url: String,
    pub remaining_targets: Vec<FileLinkTarget>,
}

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
    pub linked_targets: Vec<FileLinkTarget>,
    pub created_at: i64,
    pub updated_at: i64,
}
