use crate::{types::NotificationEntityType, *};

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct MarkAsReadRequest {
    pub notification_ids: Vec<NotificationEntityType>,
}
