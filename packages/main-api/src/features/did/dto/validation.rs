use crate::aide::OperationIo;
use crate::features::did::types::{DidDocument, VerificationMethod};
use bdk::prelude::*;

/// Request to validate a DID document
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct ValidateDidDocumentRequest {
    /// The DID document to validate
    #[serde(rename = "didDocument")]
    pub did_document: DidDocument,
}

/// Response containing validation results
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct ValidateDidDocumentResponse {
    /// Whether the document is valid
    pub valid: bool,

    /// List of validation errors (empty if valid)
    pub errors: Vec<String>,

    /// List of validation warnings (may be present even if valid)
    pub warnings: Vec<String>,
}

/// Request to verify a signature using a DID
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct VerifySignatureRequest {
    /// The DID of the signer
    pub did: String,

    /// The verification method ID (e.g., "did:web:example.com#key-1")
    #[serde(rename = "verificationMethod")]
    pub verification_method: String,

    /// The message that was signed (base64 encoded)
    pub message: String,

    /// The signature to verify (base64 encoded)
    pub signature: String,
}

/// Response containing signature verification result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct VerifySignatureResponse {
    /// Whether the signature is valid
    pub valid: bool,

    /// The verification method that was used
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "verificationMethod")]
    pub verification_method: Option<VerificationMethod>,

    /// Error message if verification failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
