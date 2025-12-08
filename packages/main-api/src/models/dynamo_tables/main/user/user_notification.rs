use crate::{utils::time::get_now_timestamp_millis, *};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserNotification {
    #[dynamo(
        prefix = "USER_NOTIFICATION",
        index = "gsi2",
        name = "find_user_notifications_by_user",
        pk
    )]
    pub pk: Partition,

    pub sk: EntityType,

    #[dynamo(
        prefix = "DEVICE_TOKEN",
        index = "gsi1",
        name = "find_user_notifications_by_token",
        pk
    )]
    pub device_token: String,

    pub platform: NotificationPlatform,
    pub enabled: bool,
    pub created_at: i64,

    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi1", sk)]
    pub updated_at: i64,

    pub last_used_at: Option<i64>,
}

#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPlatform {
    #[default]
    Android,
    Ios,
    Web,
}

impl UserNotification {
    pub fn new(
        user_pk: Partition,
        device_token: String,
        platform: NotificationPlatform,
        device_id: String,
    ) -> Self {
        let now = get_now_timestamp_millis();

        Self {
            pk: user_pk,
            sk: EntityType::UserNotification(device_id),
            device_token,
            platform,
            enabled: true,
            created_at: now,
            updated_at: now,
            last_used_at: None,
        }
    }

    pub fn touch(&mut self) {
        let now = get_now_timestamp_millis();
        self.last_used_at = Some(now);
        self.updated_at = now;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.updated_at = get_now_timestamp_millis();
    }

    pub async fn find_latest_by_user(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> Result<Option<Self>> {
        let opt = Self::opt_one().scan_index_forward(false);
        let (items, _) = Self::find_user_notifications_by_user(cli, user_pk, opt).await?;
        Ok(items.into_iter().filter(|d| d.enabled).next())
    }
}
