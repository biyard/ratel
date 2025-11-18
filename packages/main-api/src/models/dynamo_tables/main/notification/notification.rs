use crate::{
    types::{email_operation::EmailOperation, notification_status::NotificationStatus},
    utils::time::get_now_timestamp_millis,
    *,
};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, DynamoEntity, Default)]
pub struct Notification {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", pk, name = "find_by_user_notifications", order = 0)]
    #[dynamo(
        index = "gsi2",
        pk,
        prefix = "NOTI_USER",
        name = "find_by_user_notifications_by_status",
        order = 0
    )]
    pub user_pk: Partition,

    #[dynamo(index = "gsi1", sk, prefix = "TS", order = 0)]
    #[dynamo(index = "gsi2", sk, order = 1)]
    pub created_at: i64,
    pub readed_at: Option<i64>,

    #[dynamo(index = "gsi2", sk, order = 0)]
    pub status: NotificationStatus,
    pub operation: EmailOperation,
}

impl Notification {
    pub fn new(operation: EmailOperation, User { pk: user_pk, .. }: User) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let now = get_now_timestamp_millis();

        Self {
            pk: Partition::Notification(user_pk.to_string()),
            sk: EntityType::Notification(uid),
            user_pk,
            created_at: now,
            readed_at: None,

            status: NotificationStatus::Unread,
            operation,
        }
    }
}
