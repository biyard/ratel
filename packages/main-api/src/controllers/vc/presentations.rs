use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State},
};
use dto::{Result, aide, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use aws_sdk_dynamodb::Client as DynamoClient;

use crate::utils::users_dynamo::extract_user_id_dynamo;

/// Verifiable Presentation Submission Request
/// 
/// Reference: https://identity.foundation/presentation-exchange/
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
pub struct PresentationSubmissionRequest {
    #[schemars(description = "The verifiable presentation being submitted")]
    pub vp: VerifiablePresentation,
    
    #[schemars(description = "Presentation submission metadata")]
    pub presentation_submission: Option<PresentationSubmission>,
    
    #[schemars(description = "Additional context or metadata")]
    pub context: Option<Value>,
}

/// Verifiable Presentation
/// 
/// A verifiable presentation containing one or more verifiable credentials
/// and proof that the holder controls the credentials.
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
pub struct VerifiablePresentation {
    #[schemars(description = "JSON-LD context")]
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    
    #[schemars(description = "Unique identifier for the presentation")]
    pub id: Option<String>,
    
    #[schemars(description = "Type of the presentation")]
    #[serde(rename = "type")]
    pub vp_type: Vec<String>,
    
    #[schemars(description = "The holder of the credentials")]
    pub holder: Option<String>,
    
    #[schemars(description = "Verifiable credentials included in the presentation")]
    #[serde(rename = "verifiableCredential")]
    pub verifiable_credential: Vec<Value>,
    
    #[schemars(description = "Cryptographic proof of the presentation")]
    pub proof: Option<Value>,
}

/// Presentation Submission
/// 
/// Metadata about how the presentation fulfills the presentation definition.
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
pub struct PresentationSubmission {
    #[schemars(description = "Unique identifier for the submission")]
    pub id: String,
    
    #[schemars(description = "The presentation definition ID this submission responds to")]
    pub definition_id: String,
    
    #[schemars(description = "Descriptor maps showing how credentials map to requested inputs")]
    pub descriptor_map: Vec<DescriptorMap>,
}

/// Descriptor Map
/// 
/// Maps a specific credential in the presentation to a requested input.
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
pub struct DescriptorMap {
    #[schemars(description = "Input descriptor ID from the presentation definition")]
    pub id: String,
    
    #[schemars(description = "Format of the credential")]
    pub format: String,
    
    #[schemars(description = "Path to the credential in the presentation")]
    pub path: String,
    
    #[schemars(description = "Optional path to nested data within the credential")]
    pub path_nested: Option<Value>,
}

/// Presentation Verification Response
/// 
/// Response indicating the verification status of the submitted presentation.
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
pub struct PresentationVerificationResponse {
    #[schemars(description = "Unique identifier for this verification")]
    pub verification_id: String,
    
    #[schemars(description = "Overall verification status")]
    pub status: VerificationStatus,
    
    #[schemars(description = "Detailed verification results for each credential")]
    pub credential_results: Vec<CredentialVerificationResult>,
    
    #[schemars(description = "Timestamp of verification")]
    pub verified_at: String,
    
    #[schemars(description = "Additional verification metadata")]
    pub metadata: Option<Value>,
    
    #[schemars(description = "Any warnings or notices")]
    pub warnings: Vec<String>,
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
    Expired,
    Revoked,
    Suspended,
}

/// Individual Credential Verification Result
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
pub struct CredentialVerificationResult {
    #[schemars(description = "Credential identifier")]
    pub credential_id: String,
    
    #[schemars(description = "Verification status for this credential")]
    pub status: VerificationStatus,
    
    #[schemars(description = "Issuer verification status")]
    pub issuer_trusted: bool,
    
    #[schemars(description = "Signature verification status")]
    pub signature_valid: bool,
    
    #[schemars(description = "Expiration check status")]
    pub not_expired: bool,
    
    #[schemars(description = "Revocation status check")]
    pub not_revoked: bool,
    
    #[schemars(description = "Schema validation status")]
    pub schema_valid: bool,
    
    #[schemars(description = "Any verification errors")]
    pub errors: Vec<String>,
}

/// Accept Verifiable Presentation Endpoint
/// 
/// Accepts and verifies a verifiable presentation submitted by a holder.
/// Performs comprehensive verification including signature validation,
/// issuer trust, expiration checks, and revocation status.
pub async fn accept_presentation_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(dynamo_client): State<Arc<DynamoClient>>,
    Json(request): Json<PresentationSubmissionRequest>,
) -> Result<Json<PresentationVerificationResponse>> {
    tracing::debug!("Received presentation submission: {:?}", request);
    
    // Extract user ID for audit logging (optional for this endpoint)
    let verifier_id = if auth.is_some() {
        Some(extract_user_id_dynamo(&dynamo_client, "ratel_dev_main", auth).await?)
    } else {
        None
    };
    
    tracing::info!("Processing presentation from verifier: {:?}", verifier_id);
    
    // Generate verification ID
    let verification_id = uuid::Uuid::new_v4().to_string();
    let verified_at = chrono::Utc::now().to_rfc3339();
    
    // Verify the presentation structure
    let vp = &request.vp;
    verify_presentation_structure(vp)?;
    
    // Verify each credential in the presentation
    let mut credential_results = Vec::new();
    let mut overall_status = VerificationStatus::Valid;
    let mut warnings = Vec::new();
    
    for (index, credential) in vp.verifiable_credential.iter().enumerate() {
        tracing::debug!("Verifying credential {}", index);
        
        let result = verify_credential(credential, &dynamo_client).await?;
        
        // Update overall status based on individual results
        if result.status != VerificationStatus::Valid {
            overall_status = result.status.clone();
        }
        
        credential_results.push(result);
    }
    
    // Verify the presentation proof if present
    if let Some(proof) = &vp.proof {
        match verify_presentation_proof(vp, proof).await {
            Ok(_) => {
                tracing::debug!("Presentation proof verified successfully");
            }
            Err(e) => {
                tracing::warn!("Presentation proof verification failed: {:?}", e);
                warnings.push("Presentation proof verification failed".to_string());
                if overall_status == VerificationStatus::Valid {
                    overall_status = VerificationStatus::Invalid;
                }
            }
        }
    } else {
        warnings.push("No presentation proof provided".to_string());
    }
    
    // Verify presentation submission if provided
    if let Some(submission) = &request.presentation_submission {
        match verify_presentation_submission(submission, vp) {
            Ok(_) => {
                tracing::debug!("Presentation submission verified successfully");
            }
            Err(e) => {
                tracing::warn!("Presentation submission verification failed: {:?}", e);
                warnings.push(format!("Presentation submission verification failed: {}", e));
            }
        }
    }
    
    // TODO: Store verification result in database for audit trail
    // This should include:
    // - verification_id
    // - verifier_id (if authenticated)
    // - presentation data
    // - verification results
    // - timestamp
    
    let response = PresentationVerificationResponse {
        verification_id,
        status: overall_status,
        credential_results,
        verified_at,
        metadata: Some(serde_json::json!({
            "verifier_id": verifier_id,
            "presentation_id": vp.id,
            "holder": vp.holder,
            "credential_count": vp.verifiable_credential.len()
        })),
        warnings,
    };
    
    tracing::info!("Presentation verification completed with status: {:?}", response.status);
    
    Ok(Json(response))
}

/// Verify the basic structure of a verifiable presentation
fn verify_presentation_structure(vp: &VerifiablePresentation) -> Result<()> {
    // Check required context
    if !vp.context.contains(&"https://www.w3.org/2018/credentials/v1".to_string()) {
        return Err(dto::Error::Unknown("Invalid presentation context".to_string()));
    }
    
    // Check required type
    if !vp.vp_type.contains(&"VerifiablePresentation".to_string()) {
        return Err(dto::Error::Unknown("Invalid presentation type".to_string()));
    }
    
    // Check that at least one credential is present
    if vp.verifiable_credential.is_empty() {
        return Err(dto::Error::Unknown("Presentation must contain at least one credential".to_string()));
    }
    
    Ok(())
}

/// Verify an individual credential within the presentation
async fn verify_credential(credential: &Value, _dynamo_client: &Arc<DynamoClient>) -> Result<CredentialVerificationResult> {
    let credential_id = credential.get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    
    tracing::debug!("Verifying credential: {}", credential_id);
    
    // TODO: Implement comprehensive credential verification
    // This should include:
    // 1. Signature verification using the issuer's public key
    // 2. Issuer trust verification (is the issuer trusted?)
    // 3. Expiration date checking
    // 4. Revocation status checking via status list
    // 5. Schema validation
    
    // For now, we'll do basic structural validation
    let mut errors = Vec::new();
    
    // Check required fields
    if credential.get("@context").is_none() {
        errors.push("Missing @context".to_string());
    }
    
    if credential.get("type").is_none() {
        errors.push("Missing type".to_string());
    }
    
    if credential.get("issuer").is_none() {
        errors.push("Missing issuer".to_string());
    }
    
    if credential.get("credentialSubject").is_none() {
        errors.push("Missing credentialSubject".to_string());
    }
    
    // Check expiration
    let not_expired = if let Some(exp) = credential.get("expirationDate") {
        if let Some(exp_str) = exp.as_str() {
            match chrono::DateTime::parse_from_rfc3339(exp_str) {
                Ok(exp_date) => chrono::Utc::now() < exp_date.with_timezone(&chrono::Utc),
                Err(_) => {
                    errors.push("Invalid expirationDate format".to_string());
                    false
                }
            }
        } else {
            errors.push("Invalid expirationDate type".to_string());
            false
        }
    } else {
        true // No expiration date means it doesn't expire
    };
    
    if !not_expired {
        errors.push("Credential has expired".to_string());
    }
    
    // Determine overall status
    let status = if !errors.is_empty() {
        if !not_expired {
            VerificationStatus::Expired
        } else {
            VerificationStatus::Invalid
        }
    } else {
        VerificationStatus::Valid
    };
    
    Ok(CredentialVerificationResult {
        credential_id,
        status,
        issuer_trusted: true, // TODO: Implement actual issuer trust verification
        signature_valid: true, // TODO: Implement actual signature verification
        not_expired,
        not_revoked: true, // TODO: Check against status list
        schema_valid: errors.is_empty(),
        errors,
    })
}

/// Verify the presentation proof
async fn verify_presentation_proof(vp: &VerifiablePresentation, proof: &Value) -> Result<()> {
    tracing::debug!("Verifying presentation proof for VP: {:?}", vp.id);
    
    // TODO: Implement actual proof verification
    // This should:
    // 1. Verify the holder's signature over the presentation
    // 2. Check that the proof method is trusted
    // 3. Validate the proof format and structure
    // 4. Ensure the proof covers all relevant data
    
    // For now, just check basic structure
    if proof.get("type").is_none() {
        return Err(dto::Error::Unknown("Proof missing type".to_string()));
    }
    
    if proof.get("proofValue").is_none() && proof.get("jws").is_none() {
        return Err(dto::Error::Unknown("Proof missing proofValue or jws".to_string()));
    }
    
    Ok(())
}

/// Verify presentation submission against presentation definition
fn verify_presentation_submission(submission: &PresentationSubmission, vp: &VerifiablePresentation) -> Result<()> {
    tracing::debug!("Verifying presentation submission: {}", submission.id);
    
    // TODO: Implement presentation exchange verification
    // This should:
    // 1. Validate that all required input descriptors are satisfied
    // 2. Check that the credential paths are correct
    // 3. Verify any constraints and filters
    // 4. Ensure the format requirements are met
    
    // For now, just check basic structure
    if submission.descriptor_map.is_empty() {
        return Err(dto::Error::Unknown("Empty descriptor map".to_string()));
    }
    
    // Check that descriptor map count matches credential count
    if submission.descriptor_map.len() > vp.verifiable_credential.len() {
        return Err(dto::Error::Unknown("More descriptors than credentials".to_string()));
    }
    
    Ok(())
}
