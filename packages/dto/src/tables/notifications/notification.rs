use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/notifications", table = notifications, action_by_id = [dismiss, update_status_to_read])]
pub struct Notification {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, one_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, type = JSONB)]
    #[serde(default)]
    pub metadata: NotificationData,

    #[api_model(summary, version = v0.1, indexed, type = INTEGER)]
    #[serde(default)]
    pub notification_type: NotificationType,

    #[api_model(summary, version = v0.1)]
    pub read: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum NotificationType {
    #[default]
    #[translate(en = "Unknown")]
    Unknown = 0,

    #[translate(en = "Invite Team")]
    InviteTeam = 1,
    #[translate(en = "Invite Discussion")]
    InviteDiscussion = 2,
    #[translate(en = "Boosting Space")]
    BoostingSpace = 3,
    #[translate(en = "Connect Network")]
    ConnectNetwork = 4,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum NotificationData {
    #[default]
    None,

    InviteTeam {
        team_id: i64,
        group_id: i64,
        image_url: Option<String>,
        description: String,
    },
    InviteDiscussion {
        discussion_id: i64,
        image_url: Option<String>,
        description: String,
    },
    BoostingSpace {
        space_id: i64,
        image_url: Option<String>,
        description: String,
    },
    ConnectNetwork {
        requester_id: i64,
        image_url: String,
        description: String,
    },
}
