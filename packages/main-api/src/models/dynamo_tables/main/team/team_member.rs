use bdk::prelude::*;
use crate::types::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[dynamo(table = "teams")]
pub struct TeamMember {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "USER", name = "find_by_user", index = "gsi1", pk)]
    pub user_id: String,
    #[dynamo(prefix = "TEAM", name = "find_by_team", index = "gsi2", pk)]
    pub team_id: String,

    pub role: TeamMemberRole,
    pub joined_at: i64,
    pub joined_by: String,
    pub last_active_at: Option<i64>,

    pub is_active: bool,
    pub notification_settings: TeamMemberNotificationSettings,
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum TeamMemberRole {
    #[default]
    Member,
    Lead,
    Admin,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct TeamMemberNotificationSettings {
    pub mention_notifications: bool,
    pub activity_notifications: bool,
    pub digest_frequency: DigestFrequency,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub enum DigestFrequency {
    #[default]
    Daily,
    Weekly,
    Never,
}

impl TeamMember {
    pub fn new(
        team_id: String,
        user_id: String,
        joined_by: String,
        role: TeamMemberRole,
    ) -> Self {
        let _member_id = uuid::Uuid::new_v4().to_string();
        let pk = Partition::Team(team_id.clone());
        let sk = EntityType::TeamMember;

        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            user_id,
            team_id,
            role,
            joined_by,
            joined_at: now,
            is_active: true,
            notification_settings: TeamMemberNotificationSettings {
                mention_notifications: true,
                activity_notifications: true,
                digest_frequency: DigestFrequency::Daily,
            },
            ..Default::default()
        }
    }

    pub fn promote_to_lead(&mut self) {
        self.role = TeamMemberRole::Lead;
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    pub fn promote_to_admin(&mut self) {
        self.role = TeamMemberRole::Admin;
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    pub fn demote_to_member(&mut self) {
        self.role = TeamMemberRole::Member;
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    pub fn reactivate(&mut self) {
        self.is_active = true;
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    pub fn update_custom_title(&mut self, title: Option<String>) {
        self.custom_title = title;
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    pub fn update_notification_settings(&mut self, settings: TeamMemberNotificationSettings) {
        self.notification_settings = settings;
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    pub fn can_manage_team(&self) -> bool {
        matches!(self.role, TeamMemberRole::Admin | TeamMemberRole::Lead)
    }

    pub fn can_invite_members(&self) -> bool {
        matches!(self.role, TeamMemberRole::Admin | TeamMemberRole::Lead)
    }
}