use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct PresentationSubmission {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(prefix = "VERIFIER", name = "find_by_verifier", index = "gsi1", pk)]
    pub verifier_id: String,
    
    #[dynamo(prefix = "HOLDER", name = "find_by_holder", index = "gsi2", pk)]
    pub holder_id: String,
    #[dynamo(prefix = "STATUS", index = "gsi2", sk)]
    pub verification_status: String,

    pub presentation_jwt: String,
    pub definition_id: String,
    pub descriptor_map: String,
    pub verification_result_id: Option<String>,
}

impl PresentationSubmission {
    pub fn new(
        submission_id: String,
        verifier_id: String,
        holder_id: String,
        presentation_jwt: String,
        definition_id: String,
        descriptor_map: String,
    ) -> Self {
        let pk = Partition::Presentation(submission_id);
        let sk = EntityType::PresentationSubmission;
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            verifier_id,
            holder_id,
            verification_status: "pending".to_string(),
            presentation_jwt,
            definition_id,
            descriptor_map,
            verification_result_id: None,
        }
    }
}
