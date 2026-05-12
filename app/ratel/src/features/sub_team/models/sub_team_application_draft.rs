use crate::common::*;
use std::collections::HashMap;

/// In-progress (unsubmitted) sub-team application — written when the
/// applicant fills the form on `/:username/sub-teams/apply` and read
/// back when they return. Cleared after a successful submit.
///
/// One draft per (applicant team, parent team) pair: pk is the
/// applicant's team partition; sk encodes the parent team id.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamApplicationDraft {
    pub pk: Partition,  // Partition::Team(applicant_team_id)
    pub sk: EntityType, // EntityType::SubTeamApplicationDraft(parent_team_id)

    pub created_at: i64,
    pub updated_at: i64,

    /// Applicant team's id (denormalized for filtering).
    pub applicant_team_id: String,
    /// Target parent team id.
    pub parent_team_id: String,
    /// Form values keyed by `field_id` — same shape as the eventual
    /// `SubmitApplicationRequest.form_values`.
    #[serde(default)]
    pub form_values: HashMap<String, serde_json::Value>,
    /// Doc agreements captured so far — `(doc_id, body_hash)` pairs.
    /// Stored as Vec for stable serialization order.
    #[serde(default)]
    pub doc_agreements: Vec<DocAgreementSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct DocAgreementSnapshot {
    pub doc_id: String,
    pub body_hash: String,
}

#[cfg(feature = "server")]
impl SubTeamApplicationDraft {
    pub fn new(
        applicant_pk: Partition,
        parent_team_id: String,
        form_values: HashMap<String, serde_json::Value>,
        doc_agreements: Vec<DocAgreementSnapshot>,
    ) -> Self {
        let applicant_team_id = match &applicant_pk {
            Partition::Team(id) => id.clone(),
            _ => String::new(),
        };
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: applicant_pk,
            sk: EntityType::SubTeamApplicationDraft(parent_team_id.clone()),
            created_at: now,
            updated_at: now,
            applicant_team_id,
            parent_team_id,
            form_values,
            doc_agreements,
        }
    }
}
