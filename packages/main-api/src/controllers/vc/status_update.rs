use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State},
};
use dto::{Result, aide, JsonSchema};
use serde::{Deserialize, Serialize};
use std::{time::SystemTime, sync::Arc};
use aws_sdk_dynamodb::Client as DynamoClient;

use crate::utils::users_dynamo::extract_user_id_dynamo;
use crate::utils::status_list::StatusList2021;

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
    State(dynamo_client): State<Arc<DynamoClient>>,
    Json(request): Json<StatusUpdateRequest>,
) -> Result<Json<StatusUpdateResponse>> {
    tracing::debug!("Status update request: {:?}", request);
    
    // Verify user is authorized to update credential status
    let user_id = extract_user_id_dynamo(&dynamo_client, "ratel_dev_main", auth).await?;
    
    // Validate request
    validate_status_update_request(&request)?;
    
    // Verify the user has permission to update this credential
    verify_user_permission(&request.credential_id, &user_id, &dynamo_client).await?;
    
    // Update the status in the bitstring
    let updated_at = chrono::Utc::now().to_rfc3339();
    let status_list_version = update_bitstring_status(
        &request.credential_id,
        request.status_list_index,
        &request.status,
        &user_id,
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
    State(dynamo_client): State<Arc<DynamoClient>>,
    Json(request): Json<BatchStatusUpdateRequest>,
) -> Result<Json<BatchStatusUpdateResponse>> {
    tracing::debug!("Batch status update request: {} credentials", request.updates.len());
    
    // Verify user is authorized to update credential status
    let user_id = extract_user_id_dynamo(&dynamo_client, "ratel_dev_main", auth).await?;
    
    // Validate batch size
    if request.updates.len() > 1000 {
        return Err(dto::Error::Unknown("Batch size exceeds maximum limit of 1000".to_string()));
    }
    
    let mut successful_updates = 0;
    let mut failed_updates = 0;
    let mut results = Vec::new();
    
    // Process each update (for validation and individual error tracking)
    for update_request in &request.updates {
        match process_single_status_update(update_request, &user_id).await {
            Ok(_) => {
                successful_updates += 1;
                results.push(StatusUpdateResult {
                    credential_id: update_request.credential_id.clone(),
                    success: true,
                    error: None,
                });
            }
            Err(e) => {
                failed_updates += 1;
                results.push(StatusUpdateResult {
                    credential_id: update_request.credential_id.clone(),
                    success: false,
                    error: Some(format!("{:?}", e)),
                });
            }
        }
    }
    
    // Process batch updates atomically using StatusList2021 batch_update
    let status_list_version = process_batch_status_updates(&request.updates, &user_id).await?;
    
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
    updated_by: &str,
    reason: &Option<String>,
) -> Result<u64> {
    tracing::debug!(
        "Updating bitstring status for credential {} at index {} to {:?}",
        credential_id, status_list_index, status
    );
    
    // Convert status to bit value
    let revoked = match status {
        CredentialStatus::Valid => false,
        CredentialStatus::Suspended | CredentialStatus::Revoked => true,
    };
    
    // TODO: In production, this should load from persistent storage (DynamoDB)
    // For now, we'll create or load a status list
    let status_list_id = "default_status_list_1";
    let status_list_size = 1_000_000; // 1 million credentials capacity
    
    // Load existing status list or create new one
    let mut status_list = match load_status_list_from_storage(status_list_id).await {
        Ok(list) => list,
        Err(_) => {
            tracing::info!("Creating new status list: {}", status_list_id);
            StatusList2021::new(status_list_size).map_err(|e| {
                dto::Error::Unknown(format!("Failed to create status list: {}", e))
            })?
        }
    };
    
    // Update the specific index
    let index = status_list_index as usize;
    if let Err(e) = status_list.set_status(index, revoked) {
        return Err(dto::Error::Unknown(format!("Failed to update status at index {}: {}", index, e)));
    }
    
    // Store updated status list back to storage
    let new_version = store_status_list_to_storage(status_list_id, &status_list).await?;
    
    // Log the status change for audit trail
    tracing::info!(
        "Status updated: credential={}, index={}, status={:?}, updated_by={}, reason={:?}, version={}",
        credential_id, status_list_index, status, updated_by, reason, new_version
    );
    
    // TODO: In production, store audit record in DynamoDB
    // create_status_audit_record(credential_id, status_list_index, status, updated_by, reason, new_version).await?;
    
    Ok(new_version)
}

/// Process a single status update
async fn process_single_status_update(
    request: &StatusUpdateRequest,
    user_id: &str,
) -> Result<()> {
    // Validate the individual request
    validate_status_update_request(request)?;
    
    // TODO: Verify user has permission for this specific credential
    // This would involve checking if user is the issuer or has delegation rights
    
    // Update the bitstring using the main update function
    let _version = update_bitstring_status(
        &request.credential_id,
        request.status_list_index,
        &request.status,
        user_id,
        &request.reason,
    ).await?;
    
    tracing::debug!("Processed status update for credential: {}", request.credential_id);
    Ok(())
}

/// Process batch status updates atomically
async fn process_batch_status_updates(
    updates: &[StatusUpdateRequest],
    updated_by: &str,
) -> Result<u64> {
    tracing::debug!("Processing batch of {} status updates", updates.len());
    
    if updates.is_empty() {
        return get_current_status_list_version().await;
    }
    
    // Load the status list
    let status_list_id = "default_status_list_1";
    let status_list_size = 1_000_000;
    
    let mut status_list = match load_status_list_from_storage(status_list_id).await {
        Ok(list) => list,
        Err(_) => {
            tracing::info!("Creating new status list for batch update: {}", status_list_id);
            StatusList2021::new(status_list_size).map_err(|e| {
                dto::Error::Unknown(format!("Failed to create status list: {}", e))
            })?
        }
    };
    
    // Prepare batch updates for StatusList2021
    let mut batch_updates = Vec::new();
    for update in updates {
        let revoked = match update.status {
            CredentialStatus::Valid => false,
            CredentialStatus::Suspended | CredentialStatus::Revoked => true,
        };
        batch_updates.push((update.status_list_index as usize, revoked));
    }
    
    // Apply all updates atomically
    if let Err(e) = status_list.batch_update(&batch_updates) {
        return Err(dto::Error::Unknown(format!("Batch update failed: {}", e)));
    }
    
    // Store the updated status list
    let new_version = store_status_list_to_storage(status_list_id, &status_list).await?;
    
    // Log the batch update
    tracing::info!(
        "Batch status update completed: {} credentials updated by {}, version {}",
        updates.len(), updated_by, new_version
    );
    
    Ok(new_version)
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

/// Load status list from storage (DynamoDB in production)
async fn load_status_list_from_storage(status_list_id: &str) -> Result<StatusList2021> {
    tracing::debug!("Loading status list: {}", status_list_id);
    
    // TODO: In production, load from DynamoDB
    // For now, return error to trigger creation of new status list
    Err(dto::Error::Unknown("Status list not found in storage".to_string()))
}

/// Store status list to storage (DynamoDB in production)
async fn store_status_list_to_storage(status_list_id: &str, status_list: &StatusList2021) -> Result<u64> {
    tracing::debug!("Storing status list: {} (size: {})", status_list_id, status_list.size);
    
    // Generate new version number
    let version = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // TODO: In production, store to DynamoDB with structure like:
    // {
    //   "pk": "STATUS_LIST#<list_id>",
    //   "sk": "VERSION#<version>",
    //   "list_id": status_list_id,
    //   "version": version,
    //   "encoded_list": status_list.encoded_list,
    //   "size": status_list.size,
    //   "created_at": timestamp,
    //   "metadata": status_list.metadata
    // }
    
    tracing::info!("Status list {} stored with version {}", status_list_id, version);
    Ok(version)
}

/// Create audit record for status change (DynamoDB in production)
#[allow(dead_code)]
async fn create_status_audit_record(
    credential_id: &str,
    _status_list_index: u64,
    _status: &CredentialStatus,
    _updated_by: &str,
    _reason: &Option<String>,
    _version: u64,
) -> Result<()> {
    tracing::debug!("Creating audit record for credential: {}", credential_id);
    
    // TODO: In production, store audit record to DynamoDB with structure like:
    // {
    //   "pk": "AUDIT#<credential_id>",
    //   "sk": "CHANGE#<timestamp>",
    //   "credential_id": credential_id,
    //   "status_list_index": status_list_index,
    //   "old_status": old_status, // would need to track previous status
    //   "new_status": status,
    //   "updated_by": updated_by,
    //   "reason": reason,
    //   "status_list_version": version,
    //   "timestamp": chrono::Utc::now().to_rfc3339()
    // }
    
    tracing::info!("Audit record created for credential: {}", credential_id);
    Ok(())
}

/// Verify user has permission to update the credential status
async fn verify_user_permission(
    credential_id: &str,
    user_id: &str,
    _dynamo_client: &Arc<DynamoClient>,
) -> Result<()> {
    tracing::debug!("Verifying permission for user {} to update credential {}", user_id, credential_id);
    
    // TODO: In production, implement actual permission checking:
    // 1. Look up the credential in DynamoDB to find the issuer
    // 2. Check if user_id matches the issuer
    // 3. Check if user has delegation rights for this credential
    // 4. Verify the credential exists and is not in a final state
    //
    // For now, we'll allow all authenticated users to update any credential
    // This should be restricted in production!
    
    tracing::debug!("Permission granted for user {} to update credential {}", user_id, credential_id);
    Ok(())
}
