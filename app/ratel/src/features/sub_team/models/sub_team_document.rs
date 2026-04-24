use crate::common::*;

/// Maximum stored body length (64 KB) — enforced on create / update. Keeps
/// the DynamoDB item under the 400 KB per-item ceiling with headroom for the
/// other metadata fields.
pub const SUB_TEAM_DOCUMENT_MAX_BODY_BYTES: usize = 64 * 1024;

/// A sub-team governance document (e.g., bylaws, privacy notice). Phase 1 is
/// title + plain markdown body + order + required flag; attachments and
/// versioning are deferred to Phase 2.
///
/// `body_hash` is a sha256 of the current body and is what applicants snapshot
/// into `SubTeamDocAgreement.body_hash_snapshot` at submit time. On each
/// update we recompute it, which invalidates any in-flight agreements that
/// were hashed against the previous version.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamDocument {
    pub pk: Partition,  // Partition::Team(team_id) — same team that owns the doc
    pub sk: EntityType, // EntityType::SubTeamDocument(doc_id)

    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    /// Plain markdown; rendered read-only in Phase 1.
    pub body: String,

    /// If true, applicants to this team's sub-team program must explicitly
    /// agree to this document before submitting.
    #[serde(default)]
    pub required: bool,

    /// Display order; lower values render earlier.
    #[serde(default)]
    pub order: i32,

    /// sha256 of `body` at last update — re-agreement anchor.
    pub body_hash: String,
}

#[cfg(feature = "server")]
impl SubTeamDocument {
    pub fn new(team_pk: Partition, title: String, body: String, required: bool, order: i32) -> Self {
        let doc_id = uuid::Uuid::new_v4().to_string();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let body_hash = hash_body(&body);
        Self {
            pk: team_pk,
            sk: EntityType::SubTeamDocument(doc_id),
            created_at: now,
            updated_at: now,
            title,
            body,
            required,
            order,
            body_hash,
        }
    }

    pub fn update_body(&mut self, body: String) {
        self.body_hash = hash_body(&body);
        self.body = body;
        self.updated_at = crate::common::utils::time::get_now_timestamp_millis();
    }
}

#[cfg(feature = "server")]
fn hash_body(body: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(body.as_bytes());
    format!("{:x}", hasher.finalize())
}
