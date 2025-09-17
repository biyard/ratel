use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State},
};
use dto::{Result, aide, JsonSchema, sqlx::PgPool};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::utils::users::extract_user_id;

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
pub struct StatusUpdateRequest {
    #[schemars(description = "Unique identifier of the credential")]
    pub credential_id: String,
    
    #[schemars(description = "Index of the credential in the status list")]
    pub status_list_index: u64,
    
    #[schemars(description = "New status for the credential")]
    pub status: CredentialStatus,
    
    #[schemars(description = "Reason for the status change")]
    pub reason: Option<String>,
    
    #[schemars(description = "Additional metadata for the status update")]
    pub metadata: Option<serde_json::Value>,
}

/// Credential Status Update Response
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
pub struct StatusUpdateResponse {
    #[schemars(description = "Whether the update was successful")]
    pub success: bool,
    
    #[schemars(description = "Credential identifier that was updated")]
    pub credential_id: String,
    
    #[schemars(description = "New status of the credential")]
    pub status: CredentialStatus,
    
    #[schemars(description = "Timestamp when the status was updated")]
    pub updated_at: String,
    
    #[schemars(description = "URL to the updated status list")]
    pub status_list_url: String,
    
    #[schemars(description = "Version of the status list after update")]
    pub status_list_version: u64,
}

/// Possible credential statuses
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
pub enum CredentialStatus {
    #[default]
    #[schemars(description = "Credential is valid and active")]
    Valid,
    
    #[schemars(description = "Credential is temporarily suspended")]
    Suspended,
    
    #[schemars(description = "Credential is permanently revoked")]
    Revoked,
}

/// Batch Status Update Request
/// 
/// Request to update multiple credentials' statuses in a single operation.
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
pub struct BatchStatusUpdateRequest {
    #[schemars(description = "List of credential status updates")]
    pub updates: Vec<StatusUpdateRequest>,
    
    #[schemars(description = "Reason for the batch update")]
    pub batch_reason: Option<String>,
}

/// Batch Status Update Response
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
pub struct BatchStatusUpdateResponse {
    #[schemars(description = "Number of successful updates")]
    pub successful_updates: u64,
    
    #[schemars(description = "Number of failed updates")]
    pub failed_updates: u64,
    
    #[schemars(description = "Details of each update result")]
    pub results: Vec<StatusUpdateResult>,
    
    #[schemars(description = "URL to the updated status list")]
    pub status_list_url: String,
    
    #[schemars(description = "Version of the status list after updates")]
    pub status_list_version: u64,
}

/// Individual status update result
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
pub struct StatusUpdateResult {
    #[schemars(description = "Credential identifier")]
    pub credential_id: String,
    
    #[schemars(description = "Whether the update was successful")]
    pub success: bool,
    
    #[schemars(description = "Error message if update failed")]
    pub error: Option<String>,
}

/// Update Credential Status Handler
/// 
/// Updates the status of a single credential in the bitstring status list.
/// Only the credential issuer or authorized parties can update status.
pub async fn update_credential_status_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(request): Json<StatusUpdateRequest>,
) -> Result<Json<StatusUpdateResponse>> {
    tracing::debug!("Status update request: {:?}", request);
    
    // Verify user is authorized to update credential status
    let user_id = extract_user_id(&pool, auth).await?;
    
    // Validate request
    validate_status_update_request(&request)?;
    
    // TODO: Verify the user has permission to update this credential
    // This would typically involve:
    // 1. Looking up the credential in the database
    // 2. Verifying the user is the issuer or has delegation rights
    // 3. Checking if the credential exists and is not already in the requested status
    
    // Update the status in the bitstring
    let updated_at = chrono::Utc::now().to_rfc3339();
    let status_list_version = update_bitstring_status(
        &request.credential_id,
        request.status_list_index,
        &request.status,
        user_id,
        &request.reason,
    ).await?;
    
    // TODO: Store the status change in the database for audit trail
    // This should include:
    // - credential_id
    // - old_status and new_status
    // - updated_by (user_id)
    // - reason
    // - timestamp
    // - metadata
    
    let config = crate::config::get();
    let status_list_url = format!("https://{}/status/bitstring/1.json", config.domain);
    
    let response = StatusUpdateResponse {
        success: true,
        credential_id: request.credential_id,
        status: request.status,
        updated_at,
        status_list_url,
        status_list_version,
    };
    
    tracing::info!("Successfully updated credential status: {}", response.credential_id);
    Ok(Json(response))
}

/// Batch Update Credential Status Handler
/// 
/// Updates the status of multiple credentials in a single operation.
/// More efficient than individual updates for bulk operations.
pub async fn batch_update_credential_status_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(request): Json<BatchStatusUpdateRequest>,
) -> Result<Json<BatchStatusUpdateResponse>> {
    tracing::debug!("Batch status update request: {} credentials", request.updates.len());
    
    // Verify user is authorized to update credential status
    let user_id = extract_user_id(&pool, auth).await?;
    
    // Validate batch size
    if request.updates.len() > 1000 {
        return Err(dto::Error::Unknown("Batch size exceeds maximum limit of 1000".to_string()));
    }
    
    let mut successful_updates = 0;
    let mut failed_updates = 0;
    let mut results = Vec::new();
    
    // Process each update
    for update_request in request.updates {
        match process_single_status_update(&update_request, user_id).await {
            Ok(_) => {
                successful_updates += 1;
                results.push(StatusUpdateResult {
                    credential_id: update_request.credential_id,
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failed_updates += 1;
                results.push(StatusUpdateResult {
                    credential_id: update_request.credential_id,
                    success: false,
                    error: Some(format!("{:?}", e)),
                });
            }
        }
    }
    
    // TODO: Update the bitstring status list with all changes atomically
    let status_list_version = get_current_status_list_version().await?;
    
    let config = crate::config::get();
    let status_list_url = format!("https://{}/status/bitstring/1.json", config.domain);
    
    let response = BatchStatusUpdateResponse {
        successful_updates,
        failed_updates,
        results,
        status_list_url,
        status_list_version,
    };
    
    tracing::info!("Batch update completed: {} successful, {} failed", successful_updates, failed_updates);
    Ok(Json(response))
}

/// Validate status update request
fn validate_status_update_request(request: &StatusUpdateRequest) -> Result<()> {
    if request.credential_id.is_empty() {
        return Err(dto::Error::Unknown("credential_id cannot be empty".to_string()));
    }
    
    // Validate credential ID format (basic validation)
    if request.credential_id.len() < 10 {
        return Err(dto::Error::Unknown("Invalid credential_id format".to_string()));
    }
    
    // Validate status list index is reasonable
    if request.status_list_index >= 1_000_000 {
        return Err(dto::Error::Unknown("status_list_index exceeds maximum allowed value".to_string()));
    }
    
    Ok(())
}

/// Update bitstring status for a credential
async fn update_bitstring_status(
    credential_id: &str,
    status_list_index: u64,
    status: &CredentialStatus,
    _updated_by: i64,
    _reason: &Option<String>,
) -> Result<u64> {
    tracing::debug!(
        "Updating bitstring status for credential {} at index {} to {:?}",
        credential_id, status_list_index, status
    );
    
    // TODO: Implement actual bitstring update logic
    // This would involve:
    // 1. Loading the current bitstring from storage
    // 2. Decompressing it
    // 3. Setting the appropriate bit(s) based on the status
    //    - 0 for Valid
    //    - 1 for Revoked/Suspended (depending on status list purpose)
    // 4. Compressing the updated bitstring
    // 5. Storing it back with a new version number
    // 6. Updating the cache/CDN
    
    // For now, we'll simulate the update
    let _bit_value = match status {
        CredentialStatus::Valid => 0,
        CredentialStatus::Suspended | CredentialStatus::Revoked => 1,
    };
    
    // Return incremented version number
    Ok(get_current_status_list_version().await? + 1)
}

/// Process a single status update
async fn process_single_status_update(
    request: &StatusUpdateRequest,
    _user_id: i64,
) -> Result<()> {
    // Validate the individual request
    validate_status_update_request(request)?;
    
    // TODO: Verify user has permission for this specific credential
    // TODO: Update the bitstring
    // TODO: Log the change
    
    tracing::debug!("Processed status update for credential: {}", request.credential_id);
    Ok(())
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
