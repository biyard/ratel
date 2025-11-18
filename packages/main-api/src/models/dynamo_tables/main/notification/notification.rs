use crate::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, DynamoEntity, Default)]
pub struct Notification {
    pub pk: Partition,
    pub sk: EntityType,

    pub user_id: String,

    pub created_at: i64,
    pub read_at: Option<i64>,

    pub status: NotificationStatus,
    pub operation: NotificationOperation,
}
