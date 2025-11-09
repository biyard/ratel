pub mod resolve_did;
pub mod validation;

pub use resolve_did::*;
pub use validation::*;

use bdk::prelude::*;

/// Metadata about the DID resolution process
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct ResolutionMetadata {
    /// Content type of the fetched document
    pub content_type: Option<String>,

    /// HTTP status code
    pub status_code: u16,

    /// Whether the resolution was successful
    pub success: bool,

    /// Error message if resolution failed
    pub error: Option<String>,

    /// When the document was retrieved
    pub retrieved_at: i64,
}
