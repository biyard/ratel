use crate::features::notification::NotificationResponse;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct ListNotificationsResponse {
    pub items: Vec<NotificationResponse>,
    pub bookmark: Option<String>,
}
