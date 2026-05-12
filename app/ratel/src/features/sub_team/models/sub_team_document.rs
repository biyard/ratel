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

    /// Monotonically-increasing revision; starts at 1 on create and
    /// bumps by 1 on every successful update. Surfaced in the
    /// composer's "문서 정보 · Version" row (`v{version}`).
    #[serde(default)]
    pub version: i32,

    /// Username of the last editor — denormalized for the composer
    /// "Editor" row (`@{editor_username}`). Empty on legacy rows; UI
    /// falls back to `—`.
    #[serde(default)]
    pub editor_username: String,

    /// File attachments shown next to the body in the composer and
    /// reflected as `{N} 파일 · {size}` on the docs tab. Embedded
    /// (not a separate entity) — mirrors the pattern used by
    /// `SpaceQuiz` / `SpaceFile` / discussion `SpacePost`.
    #[serde(default)]
    pub attachments: Vec<File>,

    /// Category tag attached at create-time when the doc is authored
    /// as a bylaw (`"Bylaws"`) or a club rule (`"ClubBylaws"`). Drives
    /// the bylaws page filter. Empty / `None` = regular sub-team doc
    /// (the existing "required reading" use case).
    #[serde(default)]
    pub category: Option<String>,

    /// `Post.pk` of the backing post written alongside the doc at
    /// create time. The post carries the same body + category and is
    /// the source of truth for likes/comments — the bylaws card pulls
    /// engagement counts from this post and links to it on click.
    #[serde(default)]
    pub backing_post_id: Option<String>,
}

#[cfg(feature = "server")]
impl SubTeamDocument {
    pub fn new(
        team_pk: Partition,
        title: String,
        body: String,
        required: bool,
        order: i32,
        editor_username: String,
        attachments: Vec<File>,
    ) -> Self {
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
            version: 1,
            editor_username,
            attachments,
            category: None,
            backing_post_id: None,
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
