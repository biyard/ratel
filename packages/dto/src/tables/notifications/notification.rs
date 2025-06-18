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

    #[api_model(summary, many_to_one = users, action = create)]
    pub user_id: i64,
    
    #[api_model(summary, action = create, nullable)]
    pub title: Option<String>,

    #[api_model(summary, action = create)]
    pub message: String,
    
    #[api_model(summary, action = create, nullable)]
    pub image_url: Option<String>,

    #[api_model(version = v0.1, summary, type = INTEGER)]
    pub status: NotificationStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum NotificationStatus {
    #[default]
    #[translate(en = "Unread")]
    Unread = 0,
    #[translate(en = "Read")]
    Read = 1,
}