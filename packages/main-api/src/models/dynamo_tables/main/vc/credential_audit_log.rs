use bdk::prelude::*;
use crate::types::*;

/// Credential Audit Log Entity
/// 
/// Comprehensive audit trail for all credential-related operations.
/// Provides accountability, compliance, and debugging capabilities.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct CredentialAuditLog {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "USER", name = "find_by_user", index = "gsi1", pk)]
    pub user_id: String,
    
    #[dynamo(prefix = "ACTION", name = "find_by_action", index = "gsi2", pk)]
    pub action_type: String,
    #[dynamo(prefix = "CRED", index = "gsi2", sk)]
    pub credential_id: Option<String>,

    /// The specific operation performed
    pub operation: String,
    /// Subject of the operation (credential ID, offer ID, etc.)
    pub subject_id: String,
    /// Subject type (credential, offer, status_list, etc.)
    pub subject_type: String,
    /// Previous state before the operation
    pub previous_state: Option<String>,
    /// New state after the operation
    pub new_state: Option<String>,
    /// Reason for the operation
    pub reason: Option<String>,
    /// IP address of the request
    pub ip_address: Option<String>,
    /// User agent of the request
    pub user_agent: Option<String>,
    /// Additional metadata as JSON
    pub metadata: Option<String>,
    /// Whether the operation was successful
    pub success: bool,
    /// Error message if operation failed
    pub error_message: Option<String>,
}

impl CredentialAuditLog {
    pub fn new(
        user_id: String,
        action_type: String,
        operation: String,
        subject_id: String,
        subject_type: String,
    ) -> Self {
        let audit_id = format!("audit_{}", uuid::Uuid::new_v4());
        let pk = Partition::AuditLog(audit_id);
        let sk = EntityType::CredentialAuditLog;
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            user_id,
            action_type,
            credential_id: None,
            operation,
            subject_id: subject_id.clone(),
            subject_type,
            previous_state: None,
            new_state: None,
            reason: None,
            ip_address: None,
            user_agent: None,
            metadata: None,
            success: true,
            error_message: None,
        }
    }

    /// Create audit log for credential issuance
    pub fn credential_issued(
        user_id: String,
        credential_id: String,
        credential_type: String,
        holder_id: String,
    ) -> Self {
        let mut log = Self::new(
            user_id,
            "CREDENTIAL_ISSUED".to_string(),
            "issue_credential".to_string(),
            credential_id.clone(),
            "credential".to_string(),
        );
        log.credential_id = Some(credential_id);
        log.new_state = Some("issued".to_string());
        log.metadata = Some(serde_json::json!({
            "credential_type": credential_type,
            "holder_id": holder_id
        }).to_string());
        log
    }

    /// Create audit log for status change
    pub fn status_changed(
        user_id: String,
        credential_id: String,
        old_status: String,
        new_status: String,
        reason: Option<String>,
    ) -> Self {
        let mut log = Self::new(
            user_id,
            "STATUS_CHANGED".to_string(),
            "change_status".to_string(),
            credential_id.clone(),
            "credential".to_string(),
        );
        log.credential_id = Some(credential_id);
        log.previous_state = Some(old_status);
        log.new_state = Some(new_status);
        log.reason = reason;
        log
    }

    /// Create audit log for verification
    pub fn credential_verified(
        verifier_id: String,
        credential_id: Option<String>,
        verification_result: bool,
        verification_details: String,
    ) -> Self {
        let mut log = Self::new(
            verifier_id,
            "CREDENTIAL_VERIFIED".to_string(),
            "verify_credential".to_string(),
            credential_id.clone().unwrap_or_else(|| "unknown".to_string()),
            "credential".to_string(),
        );
        log.credential_id = credential_id;
        log.success = verification_result;
        log.metadata = Some(verification_details);
        log
    }

    /// Mark the operation as failed
    pub fn mark_failed(&mut self, error_message: String) {
        self.success = false;
        self.error_message = Some(error_message);
    }

    /// Add request context
    pub fn with_request_context(&mut self, ip_address: Option<String>, user_agent: Option<String>) {
        self.ip_address = ip_address;
        self.user_agent = user_agent;
    }
}
