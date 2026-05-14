use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// Immutable snapshot of a `SubTeamDocument` at a specific version.
///
/// Written on every successful create/update of the parent document:
/// on create, a v1 snapshot is laid down alongside the doc row; on
/// update, the controller bumps the doc's `version` and writes a new
/// snapshot row at that new version. Snapshots are never mutated and
/// never deleted by the regular doc lifecycle — only the deletion of
/// the parent doc cleans them up (handled in `delete_sub_team_doc`).
///
/// Sort key encodes `{doc_id}#{version:08}` so a single Dynamo query
/// can fetch every snapshot of one document in chronological order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamDocumentVersion {
    pub pk: Partition,  // Partition::Team(team_id) — same parent team
    pub sk: EntityType, // EntityType::SubTeamDocumentVersion(doc_id, padded_version)

    /// Wall-clock timestamp when this snapshot was taken (millis).
    pub created_at: i64,

    /// Denormalized doc id — convenient for client filtering when
    /// querying across a doc list without parsing the sk string.
    pub doc_id: String,

    /// Monotonic version number, matches the parent doc's `version`
    /// at the moment this snapshot was taken.
    pub version: i32,

    // ── Snapshot of the doc fields at this version ────────────────
    pub title: String,
    pub body: String,
    pub body_hash: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub order: i32,
    #[serde(default)]
    pub attachments: Vec<File>,
    /// Username of the editor who produced this version.
    #[serde(default)]
    pub editor_username: String,
}

#[cfg(feature = "server")]
impl SubTeamDocumentVersion {
    /// Build a new snapshot for a freshly-created or freshly-updated
    /// parent doc. The caller is responsible for ensuring `version`
    /// matches the doc's current `version`.
    pub fn snapshot_of(team_pk: Partition, doc: &super::SubTeamDocument) -> Self {
        let doc_id = match &doc.sk {
            EntityType::SubTeamDocument(id) => id.clone(),
            _ => String::new(),
        };
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: team_pk,
            sk: EntityType::SubTeamDocumentVersion(doc_id.clone(), pad_version(doc.version)),
            created_at: now,
            doc_id,
            version: doc.version,
            title: doc.title.clone(),
            body: doc.body.clone(),
            body_hash: doc.body_hash.clone(),
            required: doc.required,
            order: doc.order,
            attachments: doc.attachments.clone(),
            editor_username: doc.editor_username.clone(),
        }
    }
}

/// Zero-pad to 8 digits so the lexicographic sk order matches numeric
/// order (`v00000001 < v00000002 < … < v00000010`).
pub fn pad_version(v: i32) -> String {
    format!("{:08}", v.max(0))
}

/// Sort-key prefix for "every version of this doc" queries.
pub fn version_sk_prefix(doc_id: &str) -> String {
    format!("SUB_TEAM_DOCUMENT_VERSION#{doc_id}#")
}
