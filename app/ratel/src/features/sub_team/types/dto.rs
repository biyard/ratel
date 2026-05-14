use crate::common::*;

use crate::features::sub_team::models::{
    BroadcastTarget, SubTeamAnnouncementStatus, SubTeamApplicationStatus, SubTeamFormFieldType,
};
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

// ── Settings ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamSettingsResponse {
    pub is_parent_eligible: bool,
    pub min_sub_team_members: i32,
    #[serde(default)]
    pub min_sub_team_age_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateSubTeamSettingsRequest {
    #[serde(default)]
    pub is_parent_eligible: Option<bool>,
    #[serde(default)]
    pub min_sub_team_members: Option<i32>,
    #[serde(default)]
    pub min_sub_team_age_days: Option<i32>,
}

// ── Form fields ──────────────────────────────────────────────────

/// Attributes on the applicant team's profile that a form field can
/// auto-pull its value from. When a field has `linked_to: Some(_)`,
/// the apply page prefills it with the applicant team's matching
/// attribute and renders the field read-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum TeamProfileLink {
    /// `Team.display_name`
    TeamName,
    /// `Team.username`
    TeamUsername,
    /// `Team.description`
    TeamBio,
    /// `Team.profile_url`
    TeamProfileUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamFormFieldResponse {
    pub id: String,
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    pub required: bool,
    pub order: i32,
    pub options: Vec<String>,
    #[serde(default)]
    pub linked_to: Option<TeamProfileLink>,
    /// `true` for system-seeded default fields (e.g. "팀 이름",
    /// "설립 목적"). The form builder hides edit/delete controls for
    /// these rows; the public apply page treats them like normal
    /// linked fields.
    #[serde(default)]
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CreateSubTeamFormFieldRequest {
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub linked_to: Option<TeamProfileLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateSubTeamFormFieldRequest {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub field_type: Option<SubTeamFormFieldType>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
    /// `Some(link)` sets a link; `None` leaves the field unchanged.
    /// Clearing an existing link via the patch endpoint isn't
    /// supported today — admins delete and recreate the field
    /// instead. (`DynamoEntity` setters for `Option<T>` fields take
    /// the inner `T` and wrap, so the schema doesn't have a clear-
    /// variant either.)
    #[serde(default)]
    pub linked_to: Option<TeamProfileLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ReorderFormFieldsRequest {
    pub field_ids: Vec<String>,
}

// ── Documents ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamDocumentResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub required: bool,
    pub order: i32,
    pub body_hash: String,
    pub updated_at: i64,
    #[serde(default)]
    pub version: i32,
    #[serde(default)]
    pub editor_username: String,
    #[serde(default)]
    pub attachments: Vec<File>,
    /// `"Bylaws"` / `"ClubBylaws"` when this doc was authored via the
    /// bylaws page. Empty / `None` for regular sub-team docs.
    #[serde(default)]
    pub category: Option<String>,
    /// `Post.pk` of the backing post (carries the same body + category).
    /// Card click on the bylaws page routes to this post's detail page.
    #[serde(default)]
    pub backing_post_id: Option<String>,
    /// Likes on the backing post — populated server-side by the list /
    /// get handlers (batch-get from `Post.likes`). 0 when no backing
    /// post exists (regular sub-team docs).
    #[serde(default)]
    pub likes: i64,
    /// Comments on the backing post.
    #[serde(default)]
    pub comments: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CreateSubTeamDocumentRequest {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub attachments: Option<Vec<File>>,
    /// Optional category — when set to `"Bylaws"` / `"ClubBylaws"`,
    /// the handler also writes a backing Post with the same body so
    /// the bylaws page can show likes/comments + deep-link to the
    /// post detail.
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateSubTeamDocumentRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub order: Option<i32>,
    /// Replaces the attachment list when present. `None` leaves the
    /// list unchanged; `Some(vec![])` clears it.
    #[serde(default)]
    pub attachments: Option<Vec<File>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ReorderDocumentsRequest {
    pub doc_ids: Vec<String>,
}

// ── Document versions (immutable history) ───────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamDocumentVersionResponse {
    /// Parent doc id (same value across every snapshot for one doc).
    pub doc_id: String,
    pub version: i32,
    pub created_at: i64,
    pub title: String,
    pub body: String,
    pub body_hash: String,
    pub required: bool,
    pub order: i32,
    pub editor_username: String,
    pub attachments: Vec<File>,
}

// ── Public apply context ────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ApplyContextDocument {
    pub id: String,
    pub title: String,
    pub body: String,
    pub body_hash: String,
    pub order: i32,
    /// Applicants must Agree to this doc before submitting. `false`
    /// means the doc is reference-only — shown in the list but no
    /// agree button rendered and no eligibility gate.
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ApplyContextResponse {
    pub is_parent_eligible: bool,
    pub min_sub_team_members: i32,
    #[serde(default)]
    pub min_sub_team_age_days: i32,
    pub recognized_count: i64,
    pub pending_count: i64,
    pub form_fields: Vec<SubTeamFormFieldResponse>,
    pub required_docs: Vec<ApplyContextDocument>,
}

// ── Conversions from models ─────────────────────────────────────

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamFormField> for SubTeamFormFieldResponse {
    fn from(f: crate::features::sub_team::models::SubTeamFormField) -> Self {
        let id = match &f.sk {
            EntityType::SubTeamFormField(id) => id.clone(),
            _ => String::new(),
        };
        Self {
            id,
            label: f.label,
            field_type: f.field_type,
            required: f.required,
            order: f.order,
            options: f.options,
            linked_to: f.linked_to,
            locked: f.locked,
        }
    }
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamDocument> for SubTeamDocumentResponse {
    fn from(d: crate::features::sub_team::models::SubTeamDocument) -> Self {
        let id = match &d.sk {
            EntityType::SubTeamDocument(id) => id.clone(),
            _ => String::new(),
        };
        Self {
            id,
            title: d.title,
            body: d.body,
            required: d.required,
            order: d.order,
            body_hash: d.body_hash,
            updated_at: d.updated_at,
            version: d.version.max(1),
            editor_username: d.editor_username,
            attachments: d.attachments,
            category: d.category,
            backing_post_id: d.backing_post_id,
            // Engagement counts are populated by the handler after a
            // batch-get against backing posts. Default to 0 here.
            likes: 0,
            comments: 0,
        }
    }
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamDocumentVersion>
    for SubTeamDocumentVersionResponse
{
    fn from(v: crate::features::sub_team::models::SubTeamDocumentVersion) -> Self {
        Self {
            doc_id: v.doc_id,
            version: v.version,
            created_at: v.created_at,
            title: v.title,
            body: v.body,
            body_hash: v.body_hash,
            required: v.required,
            order: v.order,
            editor_username: v.editor_username,
            attachments: v.attachments,
        }
    }
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamDocument> for ApplyContextDocument {
    fn from(d: crate::features::sub_team::models::SubTeamDocument) -> Self {
        let id = match &d.sk {
            EntityType::SubTeamDocument(id) => id.clone(),
            _ => String::new(),
        };
        Self {
            id,
            title: d.title,
            body: d.body,
            body_hash: d.body_hash,
            order: d.order,
            required: d.required,
        }
    }
}

// ── Application lifecycle ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamFormFieldSnapshotDto {
    pub field_id: String,
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    pub required: bool,
    pub order: i32,
    pub options: Vec<String>,
    /// Locked default field — sorted to the top in the status page's
    /// "Submitted answers" view.
    #[serde(default)]
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamApplicationResponse {
    pub id: String,
    pub parent_team_id: String,
    pub sub_team_id: String,
    pub submitter_user_id: String,
    pub status: SubTeamApplicationStatus,
    pub decision_reason: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub submitted_at: Option<i64>,
    pub decided_at: Option<i64>,
    pub form_snapshot: Vec<SubTeamFormFieldSnapshotDto>,
    pub form_values: std::collections::HashMap<String, serde_json::Value>,
    /// Applicant team's display name. Filled by the listing handler
    /// via a `Team::get` join so the parent's queue row can show the
    /// applicant's name + initials avatar without a per-row client fetch.
    /// Empty when the join couldn't be resolved (handler logs).
    #[serde(default)]
    pub applicant_team_display_name: String,
    /// Applicant team's `@username`. Same join as `display_name`.
    #[serde(default)]
    pub applicant_team_username: String,
    /// Member count (UserTeam rows) of the applicant team at the time
    /// of listing. `0` when the join failed.
    #[serde(default)]
    pub applicant_member_count: i64,
    /// Parent (target) team display name. Filled by the listing
    /// handler — the applicant-side status page uses it to render the
    /// feedback card's author as the parent team, not the applicant.
    #[serde(default)]
    pub parent_team_display_name: String,
    /// Parent team's `@username`.
    #[serde(default)]
    pub parent_team_username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamDocAgreementResponse {
    pub doc_id: String,
    pub doc_title_snapshot: String,
    pub body_hash_snapshot: String,
    pub agreed_at: i64,
    pub agreed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamApplicationDetailResponse {
    #[serde(flatten)]
    pub application: SubTeamApplicationResponse,
    pub doc_agreements: Vec<SubTeamDocAgreementResponse>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum ParentRelationshipStatus {
    #[default]
    Standalone,
    PendingSubTeam,
    RecognizedSubTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ParentRelationshipResponse {
    pub status: ParentRelationshipStatus,
    pub parent_team_id: Option<String>,
    pub pending_parent_team_id: Option<String>,
    pub latest_application_id: Option<String>,
    /// Display name of the parent (or pending-parent) team, filled
    /// server-side via a `Team::get` join so the HUD panel can render
    /// "서울대학교" instead of a uuid. `None` for `Standalone` or when
    /// the join failed.
    #[serde(default)]
    pub parent_team_display_name: Option<String>,
    /// `@username` of the parent (or pending-parent) team.
    #[serde(default)]
    pub parent_team_username: Option<String>,
    /// Epoch-ms when this team was recognized as a sub-team
    /// (SubTeamLink.created_at). `None` unless `status ==
    /// RecognizedSubTeam`.
    #[serde(default)]
    pub recognized_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct DocAgreementInput {
    pub doc_id: String,
    pub body_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubmitApplicationRequest {
    pub parent_team_id: String,
    pub form_values: std::collections::HashMap<String, serde_json::Value>,
    pub doc_agreements: Vec<DocAgreementInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateApplicationRequest {
    #[serde(default)]
    pub form_values: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub doc_agreements: Option<Vec<DocAgreementInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ApplicationDecisionReasonRequest {
    /// Optional welcome message for approve / reject / cancel.
    /// Defaults to empty string when the caller sends `{}`.
    #[serde(default)]
    pub reason: String,
}

// ── Application drafts (autosave) ────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SaveApplicationDraftRequest {
    pub parent_team_id: String,
    pub form_values: std::collections::HashMap<String, serde_json::Value>,
    pub doc_agreements: Vec<DocAgreementInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ApplicationDraftResponse {
    pub parent_team_id: String,
    pub form_values: std::collections::HashMap<String, serde_json::Value>,
    pub doc_agreements: Vec<DocAgreementInput>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ApplicationReturnCommentRequest {
    pub comment: String,
}

// ── Leave / deregister ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct DeregisterRequest {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct LeaveParentRequest {
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TerminationAck {
    pub ok: bool,
}

// ── Announcements ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamAnnouncementResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub html_contents: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub author_user_id: String,
    pub status: SubTeamAnnouncementStatus,
    pub target_type: BroadcastTarget,
    #[serde(default)]
    pub visibility: crate::features::posts::types::Visibility,
    #[serde(default)]
    pub space_enabled: bool,
    #[serde(default)]
    pub space_type: Option<crate::features::posts::types::SpaceType>,
    #[serde(default)]
    pub space_pk: Option<String>,
    pub fan_out_count: i32,
    /// Comment count surfaced for the management Published list. Phase 1
    /// always returns 0 (per-post comments are not aggregated up to
    /// the announcement source-of-truth yet); kept as a field on the
    /// DTO so the UI doesn't need a follow-up schema change when
    /// aggregation lands.
    #[serde(default)]
    pub comments_count: i32,
    pub created_at: i64,
    pub updated_at: i64,
    pub published_at: Option<i64>,
    /// `Some(child_team_id)` for direct-to-one-sub-team announcements
    /// (rendered on the parent's sub-team detail page); `None` for the
    /// standard broadcast-to-all flow.
    #[serde(default)]
    pub target_child_team_id: Option<String>,
    /// Fan-out Post pk in the target child's feed (raw uuid, no "FEED#"
    /// prefix). Set after `handle_announcement_published` writes the
    /// Post — used by the parent's history row to link to the actual
    /// Post detail page rather than rebuilding URLs from announcement_id.
    #[serde(default)]
    pub target_post_pk: Option<String>,
}

/// Request body for `POST /sub-teams/:sub_team_id/direct-message` — the
/// "이 하위팀에만 공지" card on the parent's sub-team detail page.
/// Single-step (no draft); publishes immediately on call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SendDirectMessageRequest {
    pub title: String,
    /// HTML body. Rendered as-is in the child team's fanned-out Post.
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CreateSubTeamAnnouncementRequest {
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub html_contents: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub space_enabled: bool,
    #[serde(default)]
    pub space_type: Option<crate::features::posts::types::SpaceType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateSubTeamAnnouncementRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub html_contents: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub space_enabled: Option<bool>,
    #[serde(default)]
    pub space_type: Option<crate::features::posts::types::SpaceType>,
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamAnnouncement> for SubTeamAnnouncementResponse {
    fn from(a: crate::features::sub_team::models::SubTeamAnnouncement) -> Self {
        Self {
            id: a.announcement_id,
            title: a.title,
            body: a.body,
            html_contents: a.html_contents,
            tags: a.tags,
            author_user_id: a.author_user_id,
            status: a.status,
            target_type: a.target_type,
            visibility: a.visibility,
            space_enabled: a.space_enabled,
            space_type: a.space_type,
            space_pk: a.space_pk,
            fan_out_count: a.fan_out_count,
            comments_count: 0,
            created_at: a.created_at,
            updated_at: a.updated_at,
            published_at: a.published_at,
            target_child_team_id: a.target_child_team_id,
            target_post_pk: a.target_post_pk,
        }
    }
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamFormFieldSnapshot>
    for SubTeamFormFieldSnapshotDto
{
    fn from(s: crate::features::sub_team::models::SubTeamFormFieldSnapshot) -> Self {
        Self {
            field_id: s.field_id,
            label: s.label,
            field_type: s.field_type,
            required: s.required,
            order: s.order,
            options: s.options,
            locked: s.locked,
        }
    }
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamApplication> for SubTeamApplicationResponse {
    fn from(a: crate::features::sub_team::models::SubTeamApplication) -> Self {
        Self {
            id: a.application_id,
            parent_team_id: a.parent_team_id,
            sub_team_id: a.sub_team_id,
            submitter_user_id: a.submitter_user_id,
            status: a.status,
            decision_reason: a.decision_reason,
            created_at: a.created_at,
            updated_at: a.updated_at,
            submitted_at: a.submitted_at,
            decided_at: a.decided_at,
            form_snapshot: a.form_snapshot.into_iter().map(Into::into).collect(),
            form_values: a.form_values,
            // Applicant + parent team joins are filled by the listing
            // handler; defaults are kept empty/zero so the `From`
            // conversion stays a pure data shuffle and never touches
            // DynamoDB.
            applicant_team_display_name: String::new(),
            applicant_team_username: String::new(),
            applicant_member_count: 0,
            parent_team_display_name: String::new(),
            parent_team_username: String::new(),
        }
    }
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamDocAgreement> for SubTeamDocAgreementResponse {
    fn from(a: crate::features::sub_team::models::SubTeamDocAgreement) -> Self {
        Self {
            doc_id: a.doc_id,
            doc_title_snapshot: a.doc_title_snapshot,
            body_hash_snapshot: a.body_hash_snapshot,
            agreed_at: a.agreed_at,
            agreed_by: a.agreed_by,
        }
    }
}

// ── Activity dashboard ─────────────────────────────────────────────

/// Time window for activity aggregation. Phase 1 only supports weekly and
/// monthly; daily is deferred to Phase 2 (per design doc Scope section).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum ActivityWindow {
    Weekly,
    #[default]
    Monthly,
}

impl ActivityWindow {
    /// Milliseconds in the window — 7 days for weekly, 30 days for monthly.
    pub fn duration_ms(&self) -> i64 {
        match self {
            ActivityWindow::Weekly => 7 * 86_400 * 1000,
            ActivityWindow::Monthly => 30 * 86_400 * 1000,
        }
    }
}

/// Fixed privacy notice rendered on the dashboard. FR-6 / AC-20 requires
/// the UI to display this at all times — we return it inline so the UI does
/// not have to make a second request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct PrivacyNotice {
    pub en: String,
    pub ko: String,
}

impl PrivacyNotice {
    pub fn default_notice() -> Self {
        Self {
            en: "This dashboard reflects public and team-shared activity only. Private posts and messages are never included."
                .to_string(),
            ko: "이 대시보드는 공개 및 팀 공유 활동만 반영합니다. 비공개 게시물과 메시지는 포함되지 않습니다."
                .to_string(),
        }
    }
}

/// Item returned by GET /sub-teams — one row per recognized sub-team.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamSummaryResponse {
    pub sub_team_id: String,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub recognized_at: i64,
    pub member_count: i64,
    /// Max(Post.updated_at) across posts within the last 30 days, if any.
    pub last_activity_at: Option<i64>,
}

/// Envelope for the sub-teams list. Uses `ListResponse` conventions (items +
/// bookmark) plus a `truncated` flag indicating the Phase 1 50-sub-team cap
/// has been hit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamListResponse {
    pub items: Vec<SubTeamSummaryResponse>,
    pub bookmark: Option<String>,
    /// True when the parent has > 50 recognized sub-teams. The list is
    /// truncated at 50 ordered by approved_at DESC (Phase 1).
    pub truncated: bool,
}

/// Numeric-only counts for the activity dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ActivityCountsResponse {
    pub window: ActivityWindow,
    pub range_start_ms: i64,
    pub range_end_ms: i64,
    pub post_count: i64,
    pub space_count: i64,
    pub active_member_count: i64,
    pub total_member_count: i64,
    pub privacy_notice: PrivacyNotice,
}

/// Sub-team detail response for the overview page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamDetailResponse {
    pub sub_team_id: String,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub recognized_at: i64,
    pub window: ActivityWindow,
    pub post_count: i64,
    pub space_count: i64,
    pub active_member_count: i64,
    pub total_member_count: i64,
    pub privacy_notice: PrivacyNotice,
}

/// Per-member drill-down row. FR-6 #38: `@handle | posts | spaces participated
/// | last active date`. `space_count_participated` counts public/team-shared
/// spaces authored by the user within the window.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct MemberActivityResponse {
    pub user_id: String,
    pub handle: String,
    pub display_name: String,
    pub profile_url: String,
    pub post_count: i64,
    pub space_count_participated: i64,
    pub last_active_at: Option<i64>,
}
