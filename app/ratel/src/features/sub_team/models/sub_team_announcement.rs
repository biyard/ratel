use crate::common::*;
use crate::features::posts::types::{SpaceType, Visibility};
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// Canonical record of a parent team's broadcast announcement. Fan-out Posts
/// in each recognized sub-team's feed are derived from this; this row is the
/// source of truth for the announcement content and lifecycle. Publish flips
/// `status` → `Published` and sets `published_at`, which in turn triggers the
/// DynamoDB-Stream-driven fan-out Lambda.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamAnnouncement {
    pub pk: Partition,  // Partition::Team(parent_team_id)
    pub sk: EntityType, // EntityType::SubTeamAnnouncement(announcement_id)

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub published_at: Option<i64>,

    pub announcement_id: String,

    pub title: String,
    /// Plain-text legacy field. Kept for back-compat with pre-rich-text
    /// rows; new content lives in `html_contents`. When both are set the
    /// fanout/UI prefer `html_contents`.
    #[serde(default)]
    pub body: String,

    /// Rich-text (HTML) body produced by the post composer's editor.
    /// Populated for all rows created after the broadcast composer
    /// upgrade; older Draft rows fall back to `body`.
    #[serde(default)]
    pub html_contents: String,

    /// Free-form tags entered in the composer's right-panel tag input.
    #[serde(default)]
    pub tags: Vec<String>,

    /// File attachments uploaded in the composer's right-panel uploader.
    /// Mirrored onto the fan-out anchor Post so apply/detail surfaces can
    /// render them next to the body — same pattern as `SubTeamDocument`.
    #[serde(default)]
    pub attachments: Vec<File>,

    /// Parent-team admin user pk who authored.
    pub author_user_id: String,

    pub status: SubTeamAnnouncementStatus,
    pub target_type: BroadcastTarget,

    /// Visibility of the fanned-out Posts. Phase 1 always Public — kept
    /// as a field so future Phase 2 can flip without a schema change.
    #[serde(default)]
    pub visibility: Visibility,

    /// When ON, publish creates a Space (pk in `space_pk`) and every
    /// fanned-out child Post references it.
    #[serde(default)]
    pub space_enabled: bool,

    /// Space type chosen in the composer when `space_enabled` is true.
    #[serde(default)]
    pub space_type: Option<SpaceType>,

    /// Populated by `publish_announcement_handler` once the Space is
    /// created — used by the fanout service so every child Post points
    /// at the same Space.
    #[serde(default)]
    pub space_pk: Option<String>,

    /// Populated by the fan-out Lambda after publish — how many sub-teams
    /// actually received this announcement.
    #[serde(default)]
    pub fan_out_count: i32,

    /// When `Some(child_team_id)` the announcement is a **direct message**
    /// to a single recognized sub-team — the fanout writes ONE Post to
    /// that child's feed instead of looping every `SubTeamLink`. Used by
    /// the parent's sub-team detail page ("이 하위팀에만 공지"). Direct
    /// messages are immutable post-publish and excluded from the
    /// broadcast tab's stats. `None` keeps the legacy broadcast-to-all
    /// behaviour.
    #[serde(default)]
    pub target_child_team_id: Option<String>,

    /// pk of the fan-out Post created by `handle_announcement_published`
    /// in the target child's feed. Populated by the fanout service via a
    /// raw `update_item` write-back so the parent's direct-msg history
    /// row can deep-link to the actual Post (its pk differs from
    /// `announcement_id`, hence the explicit field).
    #[serde(default)]
    pub target_post_pk: Option<String>,

    /// Timestamp the deferred Space-publish inbox fan-out completed.
    /// `None` for broadcasts that don't carry an attached Space, and
    /// also for space-attached broadcasts whose Space hasn't been
    /// published yet. Drives idempotency in
    /// `services::announcement_fanout::handle_space_published` so
    /// stream replays don't duplicate the inbox notification.
    #[serde(default)]
    pub broadcast_notified_at: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum SubTeamAnnouncementStatus {
    #[default]
    Draft,
    Published,
    Deleted,
}

/// Phase 1 ships `AllRecognizedSubTeams` only; subset targeting (spec FR-5 #29)
/// is a Phase 2 expansion of this enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum BroadcastTarget {
    #[default]
    AllRecognizedSubTeams,
}

#[cfg(feature = "server")]
impl SubTeamAnnouncement {
    /// Create a Draft. `target_child_team_id` is `None` for the standard
    /// broadcast-to-all flow and `Some(child_id)` for the direct-to-one
    /// flow (composed from the parent's sub-team detail page) — both
    /// share the same Draft → Publish 2-step lifecycle; the difference
    /// only shows up at publish time, where the fanout writes one Post
    /// to the target child's feed instead of every recognized sub-team.
    pub fn new_draft(
        parent_team_pk: Partition,
        title: String,
        body: String,
        author_user_id: String,
        target_child_team_id: Option<TeamPartition>,
    ) -> Self {
        let announcement_id = uuid::Uuid::new_v4().to_string();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: parent_team_pk,
            sk: EntityType::SubTeamAnnouncement(announcement_id.clone()),
            created_at: now,
            updated_at: now,
            published_at: None,
            announcement_id,
            title,
            body,
            html_contents: String::new(),
            tags: Vec::new(),
            attachments: Vec::new(),
            author_user_id,
            status: SubTeamAnnouncementStatus::Draft,
            target_type: BroadcastTarget::AllRecognizedSubTeams,
            visibility: Visibility::Public,
            space_enabled: false,
            space_type: None,
            space_pk: None,
            fan_out_count: 0,
            target_child_team_id: target_child_team_id.map(|p| p.0),
            target_post_pk: None,
            broadcast_notified_at: None,
        }
    }

}
