use crate::common::*;

/// Canonical record of a parent team's broadcast announcement. Fan-out Posts
/// in each recognized sub-team's feed are derived from this; this row is the
/// source of truth for the announcement content and lifecycle. Publish flips
/// `status` → `Published` and sets `published_at`, which in turn triggers the
/// DynamoDB-Stream-driven fan-out Lambda.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamAnnouncement {
    pub pk: Partition,  // Partition::Team(parent_team_id)
    pub sk: EntityType, // EntityType::SubTeamAnnouncement(announcement_id)

    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub published_at: Option<i64>,

    pub announcement_id: String,

    pub title: String,
    pub body: String,

    /// Parent-team admin user pk who authored.
    pub author_user_id: String,

    pub status: SubTeamAnnouncementStatus,
    pub target_type: BroadcastTarget,

    /// Populated by the fan-out Lambda after publish — how many sub-teams
    /// actually received this announcement.
    #[serde(default)]
    pub fan_out_count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SubTeamAnnouncementStatus {
    #[default]
    Draft,
    Published,
    Deleted,
}

/// Phase 1 ships `AllRecognizedSubTeams` only; subset targeting (spec FR-5 #29)
/// is a Phase 2 expansion of this enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum BroadcastTarget {
    #[default]
    AllRecognizedSubTeams,
}

#[cfg(feature = "server")]
impl SubTeamAnnouncement {
    pub fn new_draft(
        parent_team_pk: Partition,
        title: String,
        body: String,
        author_user_id: String,
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
            author_user_id,
            status: SubTeamAnnouncementStatus::Draft,
            target_type: BroadcastTarget::AllRecognizedSubTeams,
            fan_out_count: 0,
        }
    }
}
