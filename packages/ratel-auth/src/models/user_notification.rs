use crate::*;

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum NotificationPlatform {
    #[default]
    Android,
    Ios,
    Web,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserNotification {
    #[dynamo(
        prefix = "USER_NOTIFICATION",
        index = "gsi2",
        name = "find_user_notifications_by_user",
        pk
    )]
    pub pk: Partition,

    pub sk: EntityType,
    pub device_token: String,

    pub platform: NotificationPlatform,
    pub created_at: i64,

    #[dynamo(index = "gsi2", sk)]
    pub updated_at: i64,

    pub last_used_at: Option<i64>,
}

impl UserNotification {
    pub fn new(
        user_pk: Partition,
        device_token: String,
        platform: NotificationPlatform,
        device_id: String,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_millis();

        Self {
            pk: user_pk,
            sk: EntityType::UserNotification(device_id),
            device_token,
            platform,
            created_at: now,
            updated_at: now,
            last_used_at: None,
        }
    }

    pub fn touch(&mut self) {
        let now = chrono::Utc::now().timestamp_millis();
        self.last_used_at = Some(now);
        self.updated_at = now;
    }
}
