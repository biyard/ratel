use crate::aide::OperationIo;
use crate::features::did::{ResolutionMetadata, types::DidDocument};
use bdk::prelude::*;

/// Request to resolve a DID
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct ResolveDidRequest {
    /// The DID to resolve (e.g., "did:web:example.com")
    pub did: String,
}

/// Response containing the resolved DID document
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct ResolveDidResponse {
    /// The resolved DID document (as JSON value since ssi::Document doesn't implement JsonSchema)
    #[serde(rename = "didDocument")]
    pub did_document: serde_json::Value,

    /// Metadata about the resolution process
    #[serde(rename = "didResolutionMetadata")]
    pub did_resolution_metadata: ResolutionMetadata,

    /// Document metadata (optional, per W3C spec)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "didDocumentMetadata")]
    pub did_document_metadata: Option<DidDocumentMetadata>,
}

/// Metadata about the DID document itself
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct DidDocumentMetadata {
    /// When the document was created (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,

    /// When the document was last updated (Unix timestamp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,

    /// Whether the DID has been deactivated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivated: Option<bool>,

    /// The version ID of the document
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,

    /// The next version ID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nextVersionId")]
    pub next_version_id: Option<String>,

    /// Equivalent IDs for this DID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "equivalentId")]
    pub equivalent_id: Option<Vec<String>>,

    /// Canonical ID for this DID
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "canonicalId")]
    pub canonical_id: Option<String>,
}
