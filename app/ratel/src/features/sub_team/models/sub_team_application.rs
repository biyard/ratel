use crate::common::*;

use super::SubTeamFormFieldType;

/// A sub-team application to a parent team. The canonical copy lives under the
/// parent's pk (`Partition::Team(parent_team_id)`) so the parent's queue is a
/// bounded sk-prefix query. A denormalized `applicant_team_pk` field projects
/// into GSI1 so the applying team can list its own application history.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamApplication {
    pub pk: Partition,  // Partition::Team(parent_team_id)
    pub sk: EntityType, // EntityType::SubTeamApplication(application_id)

    /// Applicant team's Partition::Team(sub_team_id), projected into gsi1 pk
    /// (with a distinct prefix to keep the GSI namespace isolated).
    #[dynamo(prefix = "STAPP_BY", name = "find_by_applicant", index = "gsi1", pk)]
    pub applicant_team_pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    pub updated_at: i64,

    #[serde(default)]
    pub submitted_at: Option<i64>,
    #[serde(default)]
    pub decided_at: Option<i64>,

    /// Human-readable id (equals the uuid carried in `sk`).
    pub application_id: String,

    /// Raw parent team UUID (convenient for DTOs).
    pub parent_team_id: String,

    /// Raw sub-team UUID (convenient for DTOs).
    pub sub_team_id: String,

    /// The user who submitted on behalf of the applicant team.
    pub submitter_user_id: String,

    pub status: SubTeamApplicationStatus,

    /// Used for both Reject (reason) and Return (revision comment).
    #[serde(default)]
    pub decision_reason: Option<String>,

    /// Snapshot of the parent's form fields at submission time, so edits to
    /// the form after this submission don't invalidate it.
    #[serde(default)]
    pub form_snapshot: Vec<SubTeamFormFieldSnapshot>,

    /// field_id → applicant-entered value. JSON value so different field
    /// types (text, number, date, multi-select) share one shape.
    #[serde(default)]
    pub form_values: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SubTeamApplicationStatus {
    /// Being filled in; not yet submitted to the parent.
    #[default]
    Draft,
    /// Submitted; awaiting parent decision.
    Pending,
    Approved,
    Rejected,
    /// Parent requested revision — applicant can edit + resubmit.
    Returned,
    Cancelled,
}

/// Immutable snapshot of a single form field as it existed at submission time.
/// Mirrors `SubTeamFormField` minus ownership/index metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct SubTeamFormFieldSnapshot {
    pub field_id: String,
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    pub required: bool,
    pub order: i32,
    pub options: Vec<String>,
}

#[cfg(feature = "server")]
impl SubTeamApplication {
    pub fn new(
        parent_team_pk: Partition,
        applicant_team_pk: Partition,
        parent_team_id: String,
        sub_team_id: String,
        submitter_user_id: String,
    ) -> Self {
        let application_id = uuid::Uuid::new_v4().to_string();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: parent_team_pk,
            sk: EntityType::SubTeamApplication(application_id.clone()),
            applicant_team_pk,
            created_at: now,
            updated_at: now,
            submitted_at: None,
            decided_at: None,
            application_id,
            parent_team_id,
            sub_team_id,
            submitter_user_id,
            status: SubTeamApplicationStatus::Draft,
            decision_reason: None,
            form_snapshot: Vec::new(),
            form_values: std::collections::HashMap::new(),
        }
    }
}
