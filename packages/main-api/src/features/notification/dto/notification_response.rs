use crate::email_operation::EmailOperation;
use crate::features::notification::Notification;
use crate::notification_status::NotificationStatus;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, aide::OperationIo, JsonSchema)]
pub struct NotificationResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub readed_at: Option<i64>,

    pub status: NotificationStatus,
    pub operation: EmailOperation,
}

impl From<Notification> for NotificationResponse {
    fn from(n: Notification) -> Self {
        Self {
            pk: n.pk,
            sk: n.sk,
            created_at: n.created_at,
            readed_at: n.readed_at,
            status: n.status,
            operation: n.operation,
        }
    }
}
