use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct VerificationResult {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "SUBMISSION", name = "find_by_submission", index = "gsi1", pk)]
    pub submission_id: String,
    
    #[dynamo(prefix = "STATUS", name = "find_by_status", index = "gsi2", pk)]
    pub verification_status: String,
    #[dynamo(index = "gsi2", sk)]
    pub verified_by: String,

    pub verification_details: String,
    pub error_details: Option<String>,
    pub signature_valid: bool,
    pub issuer_trusted: bool,
    pub credential_valid: bool,
    pub not_expired: bool,
}

impl VerificationResult {
    pub fn new(
        result_id: String,
        submission_id: String,
        verification_status: String,
        verified_by: String,
        verification_details: String,
        signature_valid: bool,
        issuer_trusted: bool,
        credential_valid: bool,
        not_expired: bool,
        error_details: Option<String>,
    ) -> Self {
        let pk = Partition::Presentation(result_id);
        let sk = EntityType::VerificationResult;
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            submission_id,
            verification_status,
            verified_by,
            verification_details,
            error_details,
            signature_valid,
            issuer_trusted,
            credential_valid,
            not_expired,
        }
    }
}
