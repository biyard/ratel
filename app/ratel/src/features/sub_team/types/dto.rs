use crate::common::*;

use crate::features::sub_team::models::{
    BroadcastTarget, SubTeamAnnouncementStatus, SubTeamApplicationStatus, SubTeamFormFieldType,
};

// ── Settings ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamSettingsResponse {
    pub is_parent_eligible: bool,
    pub min_sub_team_members: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateSubTeamSettingsRequest {
    #[serde(default)]
    pub is_parent_eligible: Option<bool>,
    #[serde(default)]
    pub min_sub_team_members: Option<i32>,
}

// ── Form fields ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamFormFieldResponse {
    pub id: String,
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    pub required: bool,
    pub order: i32,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateSubTeamFormFieldRequest {
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ReorderFormFieldsRequest {
    pub field_ids: Vec<String>,
}

// ── Documents ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamDocumentResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub required: bool,
    pub order: i32,
    pub body_hash: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateSubTeamDocumentRequest {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateSubTeamDocumentRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ReorderDocumentsRequest {
    pub doc_ids: Vec<String>,
}

// ── Public apply context ────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ApplyContextDocument {
    pub id: String,
    pub title: String,
    pub body: String,
    pub body_hash: String,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ApplyContextResponse {
    pub is_parent_eligible: bool,
    pub min_sub_team_members: i32,
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
        }
    }
}

// ── Application lifecycle ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamFormFieldSnapshotDto {
    pub field_id: String,
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    pub required: bool,
    pub order: i32,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamDocAgreementResponse {
    pub doc_id: String,
    pub doc_title_snapshot: String,
    pub body_hash_snapshot: String,
    pub agreed_at: i64,
    pub agreed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamApplicationDetailResponse {
    #[serde(flatten)]
    pub application: SubTeamApplicationResponse,
    pub doc_agreements: Vec<SubTeamDocAgreementResponse>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum ParentRelationshipStatus {
    #[default]
    Standalone,
    PendingSubTeam,
    RecognizedSubTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ParentRelationshipResponse {
    pub status: ParentRelationshipStatus,
    pub parent_team_id: Option<String>,
    pub pending_parent_team_id: Option<String>,
    pub latest_application_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DocAgreementInput {
    pub doc_id: String,
    pub body_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubmitApplicationRequest {
    pub parent_team_id: String,
    pub form_values: std::collections::HashMap<String, serde_json::Value>,
    pub doc_agreements: Vec<DocAgreementInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateApplicationRequest {
    #[serde(default)]
    pub form_values: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub doc_agreements: Option<Vec<DocAgreementInput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ApplicationDecisionReasonRequest {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ApplicationReturnCommentRequest {
    pub comment: String,
}

// ── Announcements ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamAnnouncementResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub author_user_id: String,
    pub status: SubTeamAnnouncementStatus,
    pub target_type: BroadcastTarget,
    pub fan_out_count: i32,
    pub created_at: i64,
    pub updated_at: i64,
    pub published_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateSubTeamAnnouncementRequest {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateSubTeamAnnouncementRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamAnnouncement> for SubTeamAnnouncementResponse {
    fn from(a: crate::features::sub_team::models::SubTeamAnnouncement) -> Self {
        Self {
            id: a.announcement_id,
            title: a.title,
            body: a.body,
            author_user_id: a.author_user_id,
            status: a.status,
            target_type: a.target_type,
            fan_out_count: a.fan_out_count,
            created_at: a.created_at,
            updated_at: a.updated_at,
            published_at: a.published_at,
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
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
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
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamListResponse {
    pub items: Vec<SubTeamSummaryResponse>,
    pub bookmark: Option<String>,
    /// True when the parent has > 50 recognized sub-teams. The list is
    /// truncated at 50 ordered by approved_at DESC (Phase 1).
    pub truncated: bool,
}

/// Numeric-only counts for the activity dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MemberActivityResponse {
    pub user_id: String,
    pub handle: String,
    pub display_name: String,
    pub profile_url: String,
    pub post_count: i64,
    pub space_count_participated: i64,
    pub last_active_at: Option<i64>,
}
