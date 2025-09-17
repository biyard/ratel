use bdk::prelude::*;
use by_axum::axum::{Json, extract::Path};
use dto::{Result, aide, JsonSchema};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Credential Status Response
/// 
/// Response containing the current status of a verifiable credential.
/// Used by verifiers to check if a credential is still valid.
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
pub struct CredentialStatusResponse {
    #[schemars(description = "Unique identifier of the credential")]
    pub credential_id: String,
    
    #[schemars(description = "Current status of the credential")]
    pub status: CredentialStatusValue,
    
    #[schemars(description = "Index of the credential in the status list")]
    pub status_list_index: u64,
    
    #[schemars(description = "URL of the status list containing this credential")]
    pub status_list_url: String,
    
    #[schemars(description = "Purpose of the status list (revocation or suspension)")]
    pub status_purpose: String,
    
    #[schemars(description = "When the status was last updated")]
    pub last_updated: String,
    
    #[schemars(description = "Version of the status list")]
    pub status_list_version: u64,
    
    #[schemars(description = "Additional metadata about the status")]
    pub metadata: Option<serde_json::Value>,
}

/// Credential Status Values
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
pub enum CredentialStatusValue {
    #[default]
    #[schemars(description = "Credential is valid and active")]
    Valid,
    
    #[schemars(description = "Credential is temporarily suspended")]
    Suspended,
    
    #[schemars(description = "Credential is permanently revoked")]
    Revoked,
    
    #[schemars(description = "Credential status is unknown or not found")]
    Unknown,
}

/// Credential Status Details
/// 
/// Extended response with additional verification information.
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
pub struct CredentialStatusDetails {
    #[schemars(description = "Basic status information")]
    #[serde(flatten)]
    pub status: CredentialStatusResponse,
    
    #[schemars(description = "Issuer information")]
    pub issuer: IssuerInfo,
    
    #[schemars(description = "Credential verification details")]
    pub verification: VerificationInfo,
    
    #[schemars(description = "History of status changes")]
    pub status_history: Option<Vec<StatusHistoryEntry>>,
}

/// Issuer Information
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
pub struct IssuerInfo {
    #[schemars(description = "DID or identifier of the issuer")]
    pub issuer_id: String,
    
    #[schemars(description = "Display name of the issuer")]
    pub issuer_name: Option<String>,
    
    #[schemars(description = "Domain of the issuer")]
    pub issuer_domain: String,
}

/// Verification Information
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
pub struct VerificationInfo {
    #[schemars(description = "Whether the credential signature is valid")]
    pub signature_valid: bool,
    
    #[schemars(description = "Whether the issuer is trusted")]
    pub issuer_trusted: bool,
    
    #[schemars(description = "Whether the credential has expired")]
    pub expired: bool,
    
    #[schemars(description = "Credential expiration date if any")]
    pub expires_at: Option<String>,
    
    #[schemars(description = "When this verification was performed")]
    pub verified_at: String,
}

/// Status History Entry
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
pub struct StatusHistoryEntry {
    #[schemars(description = "Previous status")]
    pub old_status: CredentialStatusValue,
    
    #[schemars(description = "New status")]
    pub new_status: CredentialStatusValue,
    
    #[schemars(description = "When the status changed")]
    pub changed_at: String,
    
    #[schemars(description = "Reason for the change")]
    pub reason: Option<String>,
    
    #[schemars(description = "Who made the change")]
    pub changed_by: Option<String>,
}

/// Get Credential Status Handler
/// 
/// Fetches the current status of a verifiable credential by its ID.
/// This endpoint is used by verifiers to check if a credential is still valid.
pub async fn get_credential_status_handler(
    Path(credential_id): Path<String>,
) -> Result<Json<CredentialStatusResponse>> {
    tracing::debug!("Fetching status for credential: {}", credential_id);
    
    // Validate credential ID format
    if credential_id.is_empty() || credential_id.len() < 10 {
        return Err(dto::Error::Unknown("Invalid credential ID format".to_string()));
    }
    
    // TODO: Look up the credential in the database
    // This would involve:
    // 1. Querying the credential metadata table
    // 2. Finding the status_list_index for this credential
    // 3. Loading the appropriate bitstring status list
    // 4. Extracting the bit at the given index
    // 5. Converting bit value to status enum
    
    // For now, we'll simulate the lookup
    let (status, status_list_index) = simulate_status_lookup(&credential_id).await?;
    
    let config = crate::config::get();
    let domain = config.domain;
    let status_list_url = format!("https://{}/status/bitstring/1.json", domain);
    
    let response = CredentialStatusResponse {
        credential_id: credential_id.clone(),
        status,
        status_list_index,
        status_list_url,
        status_purpose: "revocation".to_string(),
        last_updated: chrono::Utc::now().to_rfc3339(),
        status_list_version: get_current_status_list_version().await?,
        metadata: None,
    };
    
    tracing::info!("Retrieved status for credential {}: {:?}", credential_id, response.status);
    Ok(Json(response))
}

/// Get Detailed Credential Status Handler
/// 
/// Fetches detailed status information including verification details and history.
/// This endpoint provides more comprehensive information for advanced verifiers.
pub async fn get_detailed_credential_status_handler(
    Path(credential_id): Path<String>,
) -> Result<Json<CredentialStatusDetails>> {
    tracing::debug!("Fetching detailed status for credential: {}", credential_id);
    
    // Get basic status first
    let basic_status_result = get_credential_status_handler(Path(credential_id.clone())).await?;
    let basic_status = basic_status_result.0;
    
    // TODO: Get additional verification information
    // This would involve:
    // 1. Verifying the credential signature
    // 2. Checking issuer trust status
    // 3. Validating expiration dates
    // 4. Loading status change history
    
    let config = crate::config::get();
    let domain = config.domain;
    
    let issuer = IssuerInfo {
        issuer_id: format!("did:web:{}", domain),
        issuer_name: Some("Ratel Identity Issuer".to_string()),
        issuer_domain: domain.to_string(),
    };
    
    let verification = VerificationInfo {
        signature_valid: true, // TODO: Actual signature verification
        issuer_trusted: true,  // TODO: Check trust registry
        expired: false,        // TODO: Check expiration
        expires_at: None,      // TODO: Load from credential
        verified_at: chrono::Utc::now().to_rfc3339(),
    };
    
    // TODO: Load actual status history from database
    let status_history = Some(vec![
        StatusHistoryEntry {
            old_status: CredentialStatusValue::Unknown,
            new_status: CredentialStatusValue::Valid,
            changed_at: chrono::Utc::now().to_rfc3339(),
            reason: Some("Initial issuance".to_string()),
            changed_by: Some("system".to_string()),
        }
    ]);
    
    let detailed_response = CredentialStatusDetails {
        status: basic_status,
        issuer,
        verification,
        status_history,
    };
    
    tracing::info!("Retrieved detailed status for credential: {}", credential_id);
    Ok(Json(detailed_response))
}

/// Simulate status lookup for a credential
/// 
/// In a real implementation, this would:
/// 1. Query the database for the credential
/// 2. Find its status_list_index
/// 3. Load and decompress the bitstring
/// 4. Check the bit at the given index
async fn simulate_status_lookup(credential_id: &str) -> Result<(CredentialStatusValue, u64)> {
    // TODO: Replace with actual database lookup
    
    // Simulate different statuses based on credential ID pattern
    let status = if credential_id.contains("revoked") {
        CredentialStatusValue::Revoked
    } else if credential_id.contains("suspended") {
        CredentialStatusValue::Suspended
    } else if credential_id.starts_with("vc") {
        CredentialStatusValue::Valid
    } else {
        CredentialStatusValue::Unknown
    };
    
    // Simulate status list index (in real implementation, this would come from DB)
    let status_list_index = credential_id.len() as u64 * 42; // Simple hash-like calculation
    
    Ok((status, status_list_index))
}

/// Get current status list version
async fn get_current_status_list_version() -> Result<u64> {
    // TODO: Retrieve actual version from database/storage
    // For now, return a timestamp-based version
    let timestamp = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    Ok(timestamp)
}
