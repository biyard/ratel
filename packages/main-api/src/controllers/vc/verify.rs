use bdk::prelude::*;
use by_axum::axum::Json;
use dto::{Result, aide, JsonSchema};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

use crate::config;

/// Verifiable Credential/Presentation Verification Request
/// 
/// Request body for verifying a verifiable credential or verifiable presentation.
/// Supports both individual credentials and presentations containing multiple credentials.
/// 
/// Reference: https://www.w3.org/TR/vc-data-model/
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VerifyRequest {
    #[schemars(description = "The verifiable credential or presentation to verify")]
    pub verifiable_credential: Option<serde_json::Value>,
    
    #[schemars(description = "The verifiable presentation to verify")]
    pub verifiable_presentation: Option<serde_json::Value>,
    
    #[schemars(description = "Additional verification options")]
    pub options: Option<VerificationOptions>,
}

/// Verification Options
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VerificationOptions {
    #[schemars(description = "Domain for which the verification is being performed")]
    pub domain: Option<String>,
    
    #[schemars(description = "Challenge string for proof verification")]
    pub challenge: Option<String>,
    
    #[schemars(description = "Purpose of the verification (authentication, assertionMethod, etc.)")]
    pub purpose: Option<String>,
    
    #[schemars(description = "Whether to check credential status (revocation)")]
    pub check_status: Option<bool>,
    
    #[schemars(description = "Whether to verify expiration dates")]
    pub check_expiry: Option<bool>,
    
    #[schemars(description = "Trusted issuer DIDs")]
    pub trusted_issuers: Option<Vec<String>>,
}

/// Verification Response
/// 
/// Response containing the verification results and any errors found.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VerifyResponse {
    #[schemars(description = "Whether the verification was successful")]
    pub verified: bool,
    
    #[schemars(description = "Overall verification status")]
    pub status: VerificationStatus,
    
    #[schemars(description = "Detailed verification results")]
    pub results: Vec<VerificationResult>,
    
    #[schemars(description = "Any errors encountered during verification")]
    pub errors: Vec<String>,
    
    #[schemars(description = "Warnings about the credential/presentation")]
    pub warnings: Vec<String>,
    
    #[schemars(description = "Additional metadata about the verification")]
    pub metadata: Option<VerificationMetadata>,
}

/// Verification Status
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    #[default]
    Valid,
    Invalid,
    Warning,
    Error,
}

/// Individual Verification Result
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VerificationResult {
    #[schemars(description = "Type of verification check")]
    pub check: String,
    
    #[schemars(description = "Whether this check passed")]
    pub passed: bool,
    
    #[schemars(description = "Details about the check result")]
    pub message: Option<String>,
    
    #[schemars(description = "Additional data from the check")]
    pub data: Option<serde_json::Value>,
}

/// Verification Metadata
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VerificationMetadata {
    #[schemars(description = "Issuer DID that was verified")]
    pub issuer: Option<String>,
    
    #[schemars(description = "Subject DID")]
    pub subject: Option<String>,
    
    #[schemars(description = "Credential types found")]
    pub credential_types: Vec<String>,
    
    #[schemars(description = "Verification timestamp")]
    pub verified_at: String,
    
    #[schemars(description = "Verification method used")]
    pub verification_method: Option<String>,
}

/// Verify Verifiable Credential or Presentation
/// 
/// Validates the authenticity, integrity, and proof of a verifiable credential
/// or verifiable presentation according to W3C standards.
/// 
/// Verification checks include:
/// - Signature verification
/// - Issuer DID resolution and validation
/// - Credential status (revocation) checking
/// - Expiration date validation
/// - Schema validation
/// - Proof verification
pub async fn verify_vc_handler(
    Json(request): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>> {
    tracing::debug!("VC verification request: {:?}", request);
    
    let _conf = config::get();
    let now = chrono::Utc::now().to_rfc3339();
    
    let mut results = Vec::new();
    let errors = Vec::new();
    let mut warnings = Vec::new();
    let mut overall_verified = true;
    
    // Determine what we're verifying
    let (document, doc_type) = if let Some(vc) = &request.verifiable_credential {
        (vc, "VerifiableCredential")
    } else if let Some(vp) = &request.verifiable_presentation {
        (vp, "VerifiablePresentation")
    } else {
        return Ok(Json(VerifyResponse {
            verified: false,
            status: VerificationStatus::Error,
            results: vec![],
            errors: vec!["No credential or presentation provided".to_string()],
            warnings: vec![],
            metadata: None,
        }));
    };
    
    // Extract basic information
    let issuer = extract_issuer(document);
    let subject = extract_subject(document);
    let credential_types = extract_credential_types(document);
    
    // Perform verification checks
    
    // 1. Schema validation
    let schema_check = verify_schema(document, doc_type);
    results.push(schema_check.clone());
    if !schema_check.passed {
        overall_verified = false;
    }
    
    // 2. Signature verification
    let signature_check = verify_signature(document, &issuer).await;
    results.push(signature_check.clone());
    if !signature_check.passed {
        overall_verified = false;
    }
    
    // 3. Issuer DID resolution
    let issuer_check = verify_issuer_did(&issuer).await;
    results.push(issuer_check.clone());
    if !issuer_check.passed {
        overall_verified = false;
    }
    
    // 4. Expiration check
    if request.options.as_ref().and_then(|o| o.check_expiry).unwrap_or(true) {
        let expiry_check = verify_expiration(document);
        results.push(expiry_check.clone());
        if !expiry_check.passed {
            overall_verified = false;
        }
    }
    
    // 5. Status check (revocation)
    if request.options.as_ref().and_then(|o| o.check_status).unwrap_or(true) {
        let status_check = verify_credential_status(document).await;
        results.push(status_check.clone());
        if !status_check.passed {
            overall_verified = false;
        }
    }
    
    // 6. Trusted issuer check
    if let Some(options) = &request.options {
        if let Some(trusted_issuers) = &options.trusted_issuers {
            let trust_check = verify_trusted_issuer(&issuer, trusted_issuers);
            results.push(trust_check.clone());
            if !trust_check.passed {
                warnings.push("Issuer is not in the trusted issuers list".to_string());
            }
        }
    }
    
    // 7. Proof verification for presentations
    if doc_type == "VerifiablePresentation" {
        let proof_check = verify_presentation_proof(document, &request.options).await;
        results.push(proof_check.clone());
        if !proof_check.passed {
            overall_verified = false;
        }
    }
    
    // Determine overall status
    let status = if !errors.is_empty() {
        VerificationStatus::Error
    } else if !overall_verified {
        VerificationStatus::Invalid
    } else if !warnings.is_empty() {
        VerificationStatus::Warning
    } else {
        VerificationStatus::Valid
    };
    
    let metadata = VerificationMetadata {
        issuer: Some(issuer),
        subject,
        credential_types,
        verified_at: now,
        verification_method: Some("did:web".to_string()),
    };
    
    let response = VerifyResponse {
        verified: overall_verified,
        status,
        results,
        errors,
        warnings,
        metadata: Some(metadata),
    };
    
    Ok(Json(response))
}

// Helper functions for verification

fn extract_issuer(document: &serde_json::Value) -> String {
    document.get("issuer")
        .and_then(|i| i.as_str().or_else(|| i.get("id").and_then(|id| id.as_str())))
        .unwrap_or("unknown")
        .to_string()
}

fn extract_subject(document: &serde_json::Value) -> Option<String> {
    document.get("credentialSubject")
        .and_then(|cs| cs.get("id"))
        .and_then(|id| id.as_str())
        .map(|s| s.to_string())
}

fn extract_credential_types(document: &serde_json::Value) -> Vec<String> {
    if let Some(vp) = document.get("verifiableCredential") {
        // For presentations, extract types from contained credentials
        if let Some(vcs) = vp.as_array() {
            return vcs.iter()
                .filter_map(|vc| vc.get("type"))
                .filter_map(|t| t.as_array())
                .flat_map(|types| types.iter())
                .filter_map(|t| t.as_str())
                .map(|s| s.to_string())
                .collect();
        }
    }
    
    // For credentials, extract types directly
    document.get("type")
        .and_then(|t| t.as_array())
        .map(|types| types.iter()
            .filter_map(|t| t.as_str())
            .map(|s| s.to_string())
            .collect())
        .unwrap_or_default()
}

fn verify_schema(document: &serde_json::Value, doc_type: &str) -> VerificationResult {
    // Basic schema validation
    let required_fields = match doc_type {
        "VerifiableCredential" => vec!["@context", "type", "issuer", "credentialSubject"],
        "VerifiablePresentation" => vec!["@context", "type", "verifiableCredential"],
        _ => vec![],
    };
    
    for field in required_fields {
        if !document.get(field).is_some() {
            return VerificationResult {
                check: "schema".to_string(),
                passed: false,
                message: Some(format!("Missing required field: {}", field)),
                data: None,
            };
        }
    }
    
    VerificationResult {
        check: "schema".to_string(),
        passed: true,
        message: Some("Schema validation passed".to_string()),
        data: None,
    }
}

async fn verify_signature(document: &serde_json::Value, issuer: &str) -> VerificationResult {
    // TODO: Implement actual signature verification
    // This would involve:
    // 1. Extracting the proof from the document
    // 2. Resolving the issuer's DID document
    // 3. Getting the verification method
    // 4. Verifying the signature using the public key
    
    tracing::debug!("Verifying signature for issuer: {}", issuer);
    
    if document.get("proof").is_some() {
        VerificationResult {
            check: "signature".to_string(),
            passed: true,
            message: Some("Signature verification passed (mock)".to_string()),
            data: None,
        }
    } else {
        VerificationResult {
            check: "signature".to_string(),
            passed: false,
            message: Some("No proof found in document".to_string()),
            data: None,
        }
    }
}

async fn verify_issuer_did(issuer: &str) -> VerificationResult {
    // TODO: Implement DID resolution and validation
    // This would involve:
    // 1. Resolving the DID to get the DID document
    // 2. Validating the DID document structure
    // 3. Checking if the verification methods are valid
    
    tracing::debug!("Verifying issuer DID: {}", issuer);
    
    if issuer.starts_with("did:") {
        VerificationResult {
            check: "issuer_did".to_string(),
            passed: true,
            message: Some("Issuer DID validation passed (mock)".to_string()),
            data: None,
        }
    } else {
        VerificationResult {
            check: "issuer_did".to_string(),
            passed: false,
            message: Some("Invalid DID format".to_string()),
            data: None,
        }
    }
}

fn verify_expiration(document: &serde_json::Value) -> VerificationResult {
    let now = chrono::Utc::now();
    
    // Check expiration date
    if let Some(exp_date) = document.get("expirationDate").and_then(|d| d.as_str()) {
        match chrono::DateTime::parse_from_rfc3339(exp_date) {
            Ok(exp_time) => {
                if exp_time.with_timezone(&chrono::Utc) > now {
                    VerificationResult {
                        check: "expiration".to_string(),
                        passed: true,
                        message: Some("Credential has not expired".to_string()),
                        data: None,
                    }
                } else {
                    VerificationResult {
                        check: "expiration".to_string(),
                        passed: false,
                        message: Some("Credential has expired".to_string()),
                        data: Some(serde_json::json!({ "expiration_date": exp_date })),
                    }
                }
            }
            Err(_) => VerificationResult {
                check: "expiration".to_string(),
                passed: false,
                message: Some("Invalid expiration date format".to_string()),
                data: None,
            }
        }
    } else {
        VerificationResult {
            check: "expiration".to_string(),
            passed: true,
            message: Some("No expiration date specified".to_string()),
            data: None,
        }
    }
}

async fn verify_credential_status(document: &serde_json::Value) -> VerificationResult {
    // TODO: Implement status list checking
    // This would involve:
    // 1. Extracting the credentialStatus from the document
    // 2. Fetching the status list from the statusListCredential
    // 3. Checking the bit at statusListIndex
    
    if let Some(status) = document.get("credentialStatus") {
        tracing::debug!("Checking credential status: {:?}", status);
        
        VerificationResult {
            check: "status".to_string(),
            passed: true,
            message: Some("Credential status check passed (mock)".to_string()),
            data: None,
        }
    } else {
        VerificationResult {
            check: "status".to_string(),
            passed: true,
            message: Some("No status information provided".to_string()),
            data: None,
        }
    }
}

fn verify_trusted_issuer(issuer: &str, trusted_issuers: &[String]) -> VerificationResult {
    if trusted_issuers.contains(&issuer.to_string()) {
        VerificationResult {
            check: "trusted_issuer".to_string(),
            passed: true,
            message: Some("Issuer is in trusted list".to_string()),
            data: None,
        }
    } else {
        VerificationResult {
            check: "trusted_issuer".to_string(),
            passed: false,
            message: Some("Issuer is not in trusted list".to_string()),
            data: None,
        }
    }
}

async fn verify_presentation_proof(
    document: &serde_json::Value,
    options: &Option<VerificationOptions>,
) -> VerificationResult {
    // TODO: Implement presentation proof verification
    // This would involve:
    // 1. Extracting the proof from the presentation
    // 2. Verifying the challenge and domain match the expected values
    // 3. Verifying the signature over the presentation
    
    if let Some(proof) = document.get("proof") {
        // Check challenge if provided
        if let Some(opts) = options {
            if let Some(expected_challenge) = &opts.challenge {
                if let Some(proof_challenge) = proof.get("challenge").and_then(|c| c.as_str()) {
                    if proof_challenge != expected_challenge {
                        return VerificationResult {
                            check: "presentation_proof".to_string(),
                            passed: false,
                            message: Some("Challenge mismatch".to_string()),
                            data: None,
                        };
                    }
                }
            }
            
            // Check domain if provided
            if let Some(expected_domain) = &opts.domain {
                if let Some(proof_domain) = proof.get("domain").and_then(|d| d.as_str()) {
                    if proof_domain != expected_domain {
                        return VerificationResult {
                            check: "presentation_proof".to_string(),
                            passed: false,
                            message: Some("Domain mismatch".to_string()),
                            data: None,
                        };
                    }
                }
            }
        }
        
        VerificationResult {
            check: "presentation_proof".to_string(),
            passed: true,
            message: Some("Presentation proof verification passed (mock)".to_string()),
            data: None,
        }
    } else {
        VerificationResult {
            check: "presentation_proof".to_string(),
            passed: false,
            message: Some("No proof found in presentation".to_string()),
            data: None,
        }
    }
}
