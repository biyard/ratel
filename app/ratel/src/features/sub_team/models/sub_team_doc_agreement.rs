use crate::common::*;

/// Immutable audit record: a user explicitly agreed to a specific version of
/// a specific sub-team document at application-submit time. One row per
/// (application, doc) pair. Composite sk encodes both ids so a single parent
/// team pk can hold agreements for many applications.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamDocAgreement {
    pub pk: Partition,  // Partition::Team(parent_team_id)
    pub sk: EntityType, // EntityType::SubTeamDocAgreement(application_id, doc_id)

    pub application_id: String,
    pub doc_id: String,

    /// Snapshot of the doc's title at agree time — lets the audit trail
    /// survive doc deletion and title edits.
    pub doc_title_snapshot: String,

    /// sha256 of the doc body the user actually saw. If the doc is edited
    /// after this, a new agreement is required at re-submit.
    pub body_hash_snapshot: String,

    pub agreed_at: i64,

    /// User pk of the applicant who tapped 동의하기.
    pub agreed_by: String,
}

#[cfg(feature = "server")]
impl SubTeamDocAgreement {
    pub fn new(
        parent_team_pk: Partition,
        application_id: String,
        doc_id: String,
        doc_title_snapshot: String,
        body_hash_snapshot: String,
        agreed_by: String,
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: parent_team_pk,
            sk: EntityType::SubTeamDocAgreement(application_id.clone(), doc_id.clone()),
            application_id,
            doc_id,
            doc_title_snapshot,
            body_hash_snapshot,
            agreed_at: now,
            agreed_by,
        }
    }
}
