use crate::features::did::types::{DidDocument, DidIdentifier};
use crate::reqwest::Client;
use crate::utils::time::get_now_timestamp;
use bdk::prelude::*;
use std::time::Duration;

/// DID Resolution options
pub struct ResolutionOptions {
    /// HTTP client timeout in seconds
    pub timeout_secs: u64,

    /// Maximum response size in bytes (default 1MB)
    pub max_response_size: usize,

    /// Whether to follow HTTP redirects
    pub follow_redirects: bool,
}

impl Default for ResolutionOptions {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
            max_response_size: 1024 * 1024, // 1MB
            follow_redirects: true,
        }
    }
}

/// Result of DID resolution
#[derive(Debug)]
pub struct ResolutionResult {
    /// The resolved DID document
    pub document: DidDocument,

    /// Metadata about the resolution process
    pub metadata: ResolutionMetadata,
}

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

/// DID Resolver for resolving DIDs to DID Documents
/// Currently supports did:web method
pub struct DidResolver {
    client: Client,
    options: ResolutionOptions,
}

impl DidResolver {
    /// Create a new DID resolver with default options
    pub fn new() -> Result<Self, String> {
        Self::with_options(ResolutionOptions::default())
    }

    /// Create a new DID resolver with custom options
    pub fn with_options(options: ResolutionOptions) -> Result<Self, String> {
        let client = Client::builder()
            .timeout(Duration::from_secs(options.timeout_secs))
            .redirect(if options.follow_redirects {
                crate::reqwest::redirect::Policy::limited(10)
            } else {
                crate::reqwest::redirect::Policy::none()
            })
            .user_agent("ratel-did-resolver/1.0")
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self { client, options })
    }

    /// Resolve a DID string to a DID Document
    pub async fn resolve(&self, did_str: &str) -> Result<ResolutionResult, String> {
        // Parse the DID
        let did = DidIdentifier::parse(did_str)?;

        // Validate the DID
        did.validate()?;

        // Route to appropriate resolver based on method
        match did.method {
            crate::features::did::types::DidMethod::Web => self.resolve_web(&did).await,
            _ => Err(format!("Unsupported DID method: {}", did.method)),
        }
    }

    /// Resolve a did:web identifier
    /// Spec: https://w3c-ccg.github.io/did-method-web/
    async fn resolve_web(&self, did: &DidIdentifier) -> Result<ResolutionResult, String> {
        // Construct the HTTPS URL for the DID document
        let url = did.web_document_url()?;

        tracing::debug!("Resolving did:web from URL: {}", url);

        // Fetch the DID document
        let response = self
            .client
            .get(&url)
            .header("Accept", "application/did+json, application/json")
            .send()
            .await
            .map_err(|e| format!("Failed to fetch DID document: {}", e))?;

        let status_code = response.status().as_u16();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Check HTTP status
        if !response.status().is_success() {
            return Ok(ResolutionResult {
                document: DidDocument {
                    context: crate::features::did::types::DidContext::default(),
                    id: did.to_string(),
                    also_known_as: None,
                    controller: None,
                    verification_method: None,
                    authentication: None,
                    assertion_method: None,
                    key_agreement: None,
                    capability_invocation: None,
                    capability_delegation: None,
                    service: None,
                    created: None,
                    updated: None,
                },
                metadata: ResolutionMetadata {
                    content_type,
                    status_code,
                    success: false,
                    error: Some(format!("HTTP error: {}", status_code)),
                    retrieved_at: get_now_timestamp(),
                },
            });
        }

        // Read response body with size limit
        let body = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        if body.len() > self.options.max_response_size {
            return Err(format!(
                "Response size ({} bytes) exceeds maximum allowed size ({} bytes)",
                body.len(),
                self.options.max_response_size
            ));
        }

        // Parse the DID document
        let document: DidDocument = serde_json::from_slice(&body)
            .map_err(|e| format!("Failed to parse DID document: {}", e))?;

        // Validate the document
        document.validate()?;

        // Verify that the document ID matches the requested DID
        if document.id != did.to_string_without_fragment() {
            return Err(format!(
                "DID document ID mismatch: expected '{}', got '{}'",
                did.to_string_without_fragment(),
                document.id
            ));
        }

        Ok(ResolutionResult {
            document,
            metadata: ResolutionMetadata {
                content_type,
                status_code,
                success: true,
                error: None,
                retrieved_at: get_now_timestamp(),
            },
        })
    }

    /// Check if resolution is expensive for a given DID method
    /// (e.g., requires external HTTP calls)
    pub fn is_resolution_expensive(did_str: &str) -> bool {
        if let Ok(did) = DidIdentifier::parse(did_str) {
            matches!(
                did.method,
                crate::features::did::types::DidMethod::Web
                    | crate::features::did::types::DidMethod::Plc
            )
        } else {
            true // Assume expensive if we can't parse
        }
    }
}

impl Default for DidResolver {
    fn default() -> Self {
        Self::new().expect("Failed to create default DID resolver")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_resolution_expensive() {
        assert!(DidResolver::is_resolution_expensive("did:web:example.com"));
        assert!(DidResolver::is_resolution_expensive("did:plc:abc123"));
        // did:key would not be expensive (self-contained)
        // but we haven't implemented it yet
    }

    #[tokio::test]
    async fn test_resolver_creation() {
        let resolver = DidResolver::new();
        assert!(resolver.is_ok());
    }

    #[tokio::test]
    async fn test_resolver_with_custom_options() {
        let options = ResolutionOptions {
            timeout_secs: 10,
            max_response_size: 512 * 1024,
            follow_redirects: false,
        };
        let resolver = DidResolver::with_options(options);
        assert!(resolver.is_ok());
    }
}
