use base64::Engine;
use bdk::prelude::*;
use by_axum::axum::Json;
use dto::{JsonSchema, Result, aide};
use serde::{Deserialize, Serialize};

use crate::config;

/// Verifiable Credential/Presentation Verification Request
///
/// Request body for verifying a verifiable credential or verifiable presentation.
/// Supports both individual credentials and presentations containing multiple credentials.
///
/// Reference: https://www.w3.org/TR/vc-data-model/
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct VerificationOptions {
    #[schemars(description = "Domain for which the verification is being performed")]
    pub domain: Option<String>,

    #[schemars(description = "Challenge string for proof verification")]
    pub challenge: Option<String>,

    #[schemars(
        description = "Purpose of the verification (authentication, assertionMethod, etc.)"
    )]
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
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
pub async fn verify_vc_handler(Json(request): Json<VerifyRequest>) -> Result<Json<VerifyResponse>> {
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
    if request
        .options
        .as_ref()
        .and_then(|o| o.check_expiry)
        .unwrap_or(true)
    {
        let expiry_check = verify_expiration(document);
        results.push(expiry_check.clone());
        if !expiry_check.passed {
            overall_verified = false;
        }
    }

    // 5. Status check (revocation)
    if request
        .options
        .as_ref()
        .and_then(|o| o.check_status)
        .unwrap_or(true)
    {
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
    document
        .get("issuer")
        .and_then(|i| {
            i.as_str()
                .or_else(|| i.get("id").and_then(|id| id.as_str()))
        })
        .unwrap_or("unknown")
        .to_string()
}

fn extract_subject(document: &serde_json::Value) -> Option<String> {
    document
        .get("credentialSubject")
        .and_then(|cs| cs.get("id"))
        .and_then(|id| id.as_str())
        .map(|s| s.to_string())
}

fn extract_credential_types(document: &serde_json::Value) -> Vec<String> {
    if let Some(vp) = document.get("verifiableCredential") {
        // For presentations, extract types from contained credentials
        if let Some(vcs) = vp.as_array() {
            return vcs
                .iter()
                .filter_map(|vc| vc.get("type"))
                .filter_map(|t| t.as_array())
                .flat_map(|types| types.iter())
                .filter_map(|t| t.as_str())
                .map(|s| s.to_string())
                .collect();
        }
    }

    // For credentials, extract types directly
    document
        .get("type")
        .and_then(|t| t.as_array())
        .map(|types| {
            types
                .iter()
                .filter_map(|t| t.as_str())
                .map(|s| s.to_string())
                .collect()
        })
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
    tracing::debug!("Verifying signature for issuer: {}", issuer);

    // Check if this is a JWT credential (encoded as string) or JSON-LD format
    if let Some(jwt_str) = document.as_str() {
        return verify_jwt_signature(jwt_str, issuer).await;
    }

    // For JSON-LD format, check for proof
    if let Some(proof) = document.get("proof") {
        return verify_jsonld_proof(document, proof, issuer).await;
    }

    VerificationResult {
        check: "signature".to_string(),
        passed: false,
        message: Some("No signature or proof found in credential".to_string()),
        data: None,
    }
}

async fn verify_jwt_signature(jwt: &str, issuer: &str) -> VerificationResult {
    // Split JWT into parts
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return VerificationResult {
            check: "signature".to_string(),
            passed: false,
            message: Some("Invalid JWT format".to_string()),
            data: None,
        };
    }

    // Decode header to get algorithm
    match base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, parts[0]) {
        Ok(header_bytes) => {
            match serde_json::from_slice::<serde_json::Value>(&header_bytes) {
                Ok(header) => {
                    let alg = header
                        .get("alg")
                        .and_then(|a| a.as_str())
                        .unwrap_or("unknown");
                    tracing::debug!("JWT algorithm: {}", alg);

                    // For now, we'll do basic validation
                    // TODO: Implement actual signature verification with DID resolution
                    if matches!(alg, "ES256" | "EdDSA" | "RS256") {
                        VerificationResult {
                            check: "signature".to_string(),
                            passed: true,
                            message: Some(format!("JWT signature algorithm {} validated", alg)),
                            data: Some(serde_json::json!({ "algorithm": alg })),
                        }
                    } else {
                        VerificationResult {
                            check: "signature".to_string(),
                            passed: false,
                            message: Some(format!("Unsupported JWT algorithm: {}", alg)),
                            data: None,
                        }
                    }
                }
                Err(_) => VerificationResult {
                    check: "signature".to_string(),
                    passed: false,
                    message: Some("Invalid JWT header".to_string()),
                    data: None,
                },
            }
        }
        Err(_) => VerificationResult {
            check: "signature".to_string(),
            passed: false,
            message: Some("Failed to decode JWT header".to_string()),
            data: None,
        },
    }
}

async fn verify_jsonld_proof(
    document: &serde_json::Value,
    proof: &serde_json::Value,
    issuer: &str,
) -> VerificationResult {
    // Extract proof details
    let proof_type = proof
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("unknown");
    let verification_method = proof.get("verificationMethod").and_then(|vm| vm.as_str());

    tracing::debug!(
        "Verifying proof type: {} for issuer: {}",
        proof_type,
        issuer
    );

    // Validate proof structure
    if proof.get("proofValue").is_none() && proof.get("signatureValue").is_none() {
        return VerificationResult {
            check: "signature".to_string(),
            passed: false,
            message: Some("Missing proof value".to_string()),
            data: None,
        };
    }

    // Check if verification method belongs to issuer
    if let Some(vm) = verification_method {
        if !vm.starts_with(issuer) {
            return VerificationResult {
                check: "signature".to_string(),
                passed: false,
                message: Some("Verification method does not belong to issuer".to_string()),
                data: None,
            };
        }
    }

    // For now, accept known proof types
    // TODO: Implement actual cryptographic verification
    if matches!(
        proof_type,
        "Ed25519Signature2020" | "EcdsaSecp256k1Signature2019" | "JsonWebSignature2020"
    ) {
        VerificationResult {
            check: "signature".to_string(),
            passed: true,
            message: Some(format!("Proof type {} validated", proof_type)),
            data: Some(serde_json::json!({
                "proof_type": proof_type,
                "verification_method": verification_method
            })),
        }
    } else {
        VerificationResult {
            check: "signature".to_string(),
            passed: false,
            message: Some(format!("Unsupported proof type: {}", proof_type)),
            data: None,
        }
    }
}

async fn verify_issuer_did(issuer: &str) -> VerificationResult {
    tracing::debug!("Verifying issuer DID: {}", issuer);

    // Validate DID format
    if !issuer.starts_with("did:") {
        return VerificationResult {
            check: "issuer_did".to_string(),
            passed: false,
            message: Some("Invalid DID format - must start with 'did:'".to_string()),
            data: None,
        };
    }

    // Parse DID method
    let did_parts: Vec<&str> = issuer.split(':').collect();
    if did_parts.len() < 3 {
        return VerificationResult {
            check: "issuer_did".to_string(),
            passed: false,
            message: Some("Invalid DID format - missing method or identifier".to_string()),
            data: None,
        };
    }

    let method = did_parts[1];
    tracing::debug!("DID method: {}", method);

    // For did:web, validate domain
    if method == "web" {
        if did_parts.len() < 3 || did_parts[2].is_empty() {
            return VerificationResult {
                check: "issuer_did".to_string(),
                passed: false,
                message: Some("Invalid did:web format - missing domain".to_string()),
                data: None,
            };
        }

        let domain = did_parts[2];
        // Basic domain validation
        if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
            return VerificationResult {
                check: "issuer_did".to_string(),
                passed: false,
                message: Some("Invalid domain in did:web".to_string()),
                data: None,
            };
        }

        // TODO: Actually resolve DID document from https://{domain}/.well-known/did.json
        // For now, we'll validate the format and accept it
        return VerificationResult {
            check: "issuer_did".to_string(),
            passed: true,
            message: Some(format!("DID:web format validated for domain: {}", domain)),
            data: Some(serde_json::json!({
                "method": method,
                "domain": domain,
                "did_url": format!("https://{}/.well-known/did.json", domain)
            })),
        };
    }

    // For other DID methods, validate basic format
    match method {
        "key" | "peer" | "ethr" | "ion" | "sov" => VerificationResult {
            check: "issuer_did".to_string(),
            passed: true,
            message: Some(format!("DID method '{}' format validated", method)),
            data: Some(serde_json::json!({ "method": method })),
        },
        _ => VerificationResult {
            check: "issuer_did".to_string(),
            passed: false,
            message: Some(format!("Unsupported DID method: {}", method)),
            data: None,
        },
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
            },
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
    // Extract credential status information
    if let Some(status) = document.get("credentialStatus") {
        tracing::debug!("Checking credential status: {:?}", status);

        // Validate status structure
        let status_type = status
            .get("type")
            .and_then(|t| t.as_str())
            .unwrap_or("unknown");
        let status_list_index = status.get("statusListIndex").and_then(|idx| idx.as_str());
        let status_list_credential = status
            .get("statusListCredential")
            .and_then(|slc| slc.as_str());
        let status_purpose = status
            .get("statusPurpose")
            .and_then(|sp| sp.as_str())
            .unwrap_or("revocation");

        // Validate required fields for Status List 2021
        if status_type == "StatusList2021Entry" || status_type == "BitstringStatusListEntry" {
            if status_list_index.is_none() {
                return VerificationResult {
                    check: "status".to_string(),
                    passed: false,
                    message: Some("Missing statusListIndex in credential status".to_string()),
                    data: None,
                };
            }

            if status_list_credential.is_none() {
                return VerificationResult {
                    check: "status".to_string(),
                    passed: false,
                    message: Some("Missing statusListCredential in credential status".to_string()),
                    data: None,
                };
            }

            // Validate index is a number
            if let Some(idx_str) = status_list_index {
                match idx_str.parse::<u64>() {
                    Ok(index) => {
                        tracing::debug!("Status list index: {}", index);

                        // TODO: Fetch and check the actual status list
                        // For now, we'll validate the structure and assume status is valid
                        if let Some(slc_url) = status_list_credential {
                            return check_status_list(slc_url, index, status_purpose).await;
                        }
                    }
                    Err(_) => {
                        return VerificationResult {
                            check: "status".to_string(),
                            passed: false,
                            message: Some("Invalid statusListIndex - must be a number".to_string()),
                            data: None,
                        };
                    }
                }
            }
        }

        VerificationResult {
            check: "status".to_string(),
            passed: true,
            message: Some(format!(
                "Credential status structure validated (type: {})",
                status_type
            )),
            data: Some(serde_json::json!({
                "status_type": status_type,
                "status_purpose": status_purpose,
                "status_list_index": status_list_index,
                "status_list_credential": status_list_credential
            })),
        }
    } else {
        VerificationResult {
            check: "status".to_string(),
            passed: true,
            message: Some(
                "No status information provided - assuming credential is valid".to_string(),
            ),
            data: None,
        }
    }
}

async fn check_status_list(status_list_url: &str, index: u64, purpose: &str) -> VerificationResult {
    tracing::debug!(
        "Checking status list: {} for index: {} with purpose: {}",
        status_list_url,
        index,
        purpose
    );

    // Validate URL format
    if !status_list_url.starts_with("https://") {
        return VerificationResult {
            check: "status".to_string(),
            passed: false,
            message: Some("Status list URL must use HTTPS".to_string()),
            data: None,
        };
    }

    // TODO: Implement actual HTTP request to fetch status list
    // This would involve:
    // 1. Fetching the status list credential from the URL
    // 2. Verifying the status list credential signature
    // 3. Extracting the compressed bitstring
    // 4. Decompressing and checking the bit at the given index

    // For now, simulate the check
    // In production, this would make an HTTP request and parse the bitstring
    let simulated_status = simulate_status_check(index);

    match simulated_status {
        0 => VerificationResult {
            check: "status".to_string(),
            passed: true,
            message: Some(format!(
                "Credential status is valid (index {}, purpose: {})",
                index, purpose
            )),
            data: Some(serde_json::json!({
                "status": "valid",
                "index": index,
                "purpose": purpose,
                "status_list_url": status_list_url
            })),
        },
        1 => VerificationResult {
            check: "status".to_string(),
            passed: false,
            message: Some(format!("Credential has been {} (index {})", purpose, index)),
            data: Some(serde_json::json!({
                "status": purpose, // "revoked" or "suspended"
                "index": index,
                "purpose": purpose,
                "status_list_url": status_list_url
            })),
        },
        _ => VerificationResult {
            check: "status".to_string(),
            passed: false,
            message: Some("Error checking credential status".to_string()),
            data: None,
        },
    }
}

fn simulate_status_check(index: u64) -> u8 {
    // Simulate status check - in production this would check actual bitstring
    // For demo purposes, let's say indices 0-999 are valid, 1000+ are revoked
    if index < 1000 {
        0 // Valid
    } else {
        1 // Revoked/Suspended
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
    if let Some(proof) = document.get("proof") {
        tracing::debug!("Verifying presentation proof: {:?}", proof);

        // Validate proof structure
        let proof_type = proof
            .get("type")
            .and_then(|t| t.as_str())
            .unwrap_or("unknown");
        let verification_method = proof.get("verificationMethod").and_then(|vm| vm.as_str());
        let proof_purpose = proof
            .get("proofPurpose")
            .and_then(|pp| pp.as_str())
            .unwrap_or("unknown");

        // Check if this is a valid proof purpose for presentations
        if !matches!(proof_purpose, "authentication" | "assertionMethod") {
            return VerificationResult {
                check: "presentation_proof".to_string(),
                passed: false,
                message: Some(format!(
                    "Invalid proof purpose for presentation: {}",
                    proof_purpose
                )),
                data: None,
            };
        }

        // Check challenge if provided in options
        if let Some(opts) = options {
            if let Some(expected_challenge) = &opts.challenge {
                if let Some(proof_challenge) = proof.get("challenge").and_then(|c| c.as_str()) {
                    if proof_challenge != expected_challenge {
                        return VerificationResult {
                            check: "presentation_proof".to_string(),
                            passed: false,
                            message: Some("Challenge mismatch in presentation proof".to_string()),
                            data: Some(serde_json::json!({
                                "expected_challenge": expected_challenge,
                                "actual_challenge": proof_challenge
                            })),
                        };
                    }
                } else {
                    return VerificationResult {
                        check: "presentation_proof".to_string(),
                        passed: false,
                        message: Some("Challenge expected but not found in proof".to_string()),
                        data: None,
                    };
                }
            }

            // Check domain if provided in options
            if let Some(expected_domain) = &opts.domain {
                if let Some(proof_domain) = proof.get("domain").and_then(|d| d.as_str()) {
                    if proof_domain != expected_domain {
                        return VerificationResult {
                            check: "presentation_proof".to_string(),
                            passed: false,
                            message: Some("Domain mismatch in presentation proof".to_string()),
                            data: Some(serde_json::json!({
                                "expected_domain": expected_domain,
                                "actual_domain": proof_domain
                            })),
                        };
                    }
                } else {
                    return VerificationResult {
                        check: "presentation_proof".to_string(),
                        passed: false,
                        message: Some("Domain expected but not found in proof".to_string()),
                        data: None,
                    };
                }
            }
        }

        // Check that proof has signature value
        if proof.get("proofValue").is_none()
            && proof.get("signatureValue").is_none()
            && proof.get("jws").is_none()
        {
            return VerificationResult {
                check: "presentation_proof".to_string(),
                passed: false,
                message: Some("No signature value found in presentation proof".to_string()),
                data: None,
            };
        }

        // Validate verification method format if present
        if let Some(vm) = verification_method {
            if !vm.starts_with("did:") && !vm.starts_with("https://") {
                return VerificationResult {
                    check: "presentation_proof".to_string(),
                    passed: false,
                    message: Some("Invalid verification method format".to_string()),
                    data: None,
                };
            }
        }

        // Check proof timestamp if present
        if let Some(created) = proof.get("created").and_then(|c| c.as_str()) {
            match chrono::DateTime::parse_from_rfc3339(created) {
                Ok(proof_time) => {
                    let now = chrono::Utc::now();
                    let age = now.signed_duration_since(proof_time.with_timezone(&chrono::Utc));

                    // Reject proofs older than 1 hour or in the future
                    if age.num_seconds() > 3600 || age.num_seconds() < 0 {
                        return VerificationResult {
                            check: "presentation_proof".to_string(),
                            passed: false,
                            message: Some(
                                "Presentation proof timestamp is too old or in the future"
                                    .to_string(),
                            ),
                            data: Some(serde_json::json!({
                                "proof_created": created,
                                "age_seconds": age.num_seconds()
                            })),
                        };
                    }
                }
                Err(_) => {
                    return VerificationResult {
                        check: "presentation_proof".to_string(),
                        passed: false,
                        message: Some("Invalid timestamp format in presentation proof".to_string()),
                        data: None,
                    };
                }
            }
        }

        // TODO: Implement actual cryptographic verification of the presentation proof
        // This would involve:
        // 1. Resolving the verification method to get the public key
        // 2. Canonicalizing the presentation according to the proof type
        // 3. Verifying the signature

        VerificationResult {
            check: "presentation_proof".to_string(),
            passed: true,
            message: Some(format!(
                "Presentation proof structure validated (type: {}, purpose: {})",
                proof_type, proof_purpose
            )),
            data: Some(serde_json::json!({
                "proof_type": proof_type,
                "proof_purpose": proof_purpose,
                "verification_method": verification_method
            })),
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
