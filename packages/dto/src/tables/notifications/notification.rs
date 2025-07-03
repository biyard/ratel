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


    #[api_model(summary, many_to_one = users, nullable)]
    pub target_user_id: Option<i64>,
    #[api_model(summary, one_to_one = users)]
    pub from_user_id: i64,
    

    #[api_model(summary, nullable)]
    pub title: Option<String>,
    #[api_model(summary)]
    pub metadata: String,
    #[api_model(summary, nullable)]
    pub image_url: Option<String>,
    #[api_model(summary, nullable)]
    pub profile_url: Option<String>,
    #[api_model(summary, nullable)]
    pub space_id: Option<i64>,

    #[api_model(summary, version = v0.1, indexed, type = INTEGER)]
    #[serde(default)]
    pub notification_type: NotificationType,

    #[api_model(version = v0.1, summary)]
    pub read: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum NotificationType {
    #[default]
    #[translate(en = "Unknown")]
    Unknown = 0,
    #[translate(en = "Boosting Space")]
    BoostingSpace = 1,
    #[translate(en = "Connect Network")]
    ConnectNetwork = 2,
}


// #[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
// #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
// pub enum test {
//     #[default]
//     Test =,
// }