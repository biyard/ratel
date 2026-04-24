use crate::common::*;

/// Recognized parent→child relationship record. One row per approved sub-team,
/// stored under the parent team's pk so the parent can list its sub-teams with
/// a single sk-prefix query. Created in a transact-write-items batch with the
/// child team's `parent_team_id` update on approval; deleted on deregister,
/// leave-parent, or parent deletion cascade.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamLink {
    pub pk: Partition,  // Partition::Team(parent_team_id)
    pub sk: EntityType, // EntityType::SubTeamLink(child_team_id)

    /// Raw child team UUID (convenient for callers that want it without
    /// re-parsing the sk).
    pub child_team_id: String,

    pub approved_at: i64,

    /// User pk of the parent-team admin who approved.
    pub approved_by: String,

    /// The application this link was born from.
    pub source_application_id: String,
}

#[cfg(feature = "server")]
impl SubTeamLink {
    pub fn new(parent_team_pk: Partition, child_team_id: String, approved_by: String, source_application_id: String) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: parent_team_pk,
            sk: EntityType::SubTeamLink(child_team_id.clone()),
            child_team_id,
            approved_at: now,
            approved_by,
            source_application_id,
        }
    }
}
