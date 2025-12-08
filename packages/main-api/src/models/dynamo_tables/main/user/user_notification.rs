use serde_json::json;

use crate::{
    utils::{
        aws::DynamoClient, firebase::oauth::get_fcm_access_token, time::get_now_timestamp_millis,
    },
    *,
};

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

    pub async fn send_to_user(
        dynamo: &DynamoClient,
        user_pk: &Partition,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Result<()> {
        let title = title.into();
        let body = body.into();

        let project_id = config::get().ratel_project_id;
        let fcm_enabled = config::get().fcm_enabled;

        if !fcm_enabled {
            tracing::info!(
                "UserNotification::send_to_user: FCM_ENABLED != true, skip push (project_id={})",
                project_id
            );
            return Ok(());
        }

        let Some(device) = UserNotification::find_latest_by_user(&dynamo.client, user_pk).await?
        else {
            tracing::debug!(
                "UserNotification::send_to_user: no active device for user_pk={}",
                user_pk
            );
            return Ok(());
        };

        if !device.enabled {
            tracing::debug!(
                "UserNotification::send_to_user: device disabled for user_pk={}, token_prefix={}",
                user_pk,
                &device.device_token.chars().take(10).collect::<String>()
            );
            return Ok(());
        }

        let access_token = get_fcm_access_token().await?;
        tracing::debug!(
            "UserNotification::send_to_user: got access_token (len={})",
            access_token.len()
        );

        let client = reqwest::Client::new();
        let endpoint = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            project_id
        );

        let payload = json!({
            "message": {
                "token": device.device_token,
                "notification": {
                    "title": title,
                    "body":  body,
                },
            }
        });

        tracing::debug!(
            "UserNotification::send_to_user: request payload for user_pk={}: {}",
            user_pk,
            payload
        );

        let res = client
            .post(&endpoint)
            .bearer_auth(&access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::InternalServerError(format!("FCM v1 request failed: {e}")))?;

        let status = res.status();
        let text = res.text().await.unwrap_or_default();

        if !status.is_success() {
            tracing::warn!(
                "UserNotification::send_to_user: FCM v1 push failed: status={}, body={}, user_pk={}, token_prefix={}",
                status,
                text,
                user_pk,
                &device.device_token.chars().take(10).collect::<String>()
            );
        } else {
            tracing::info!(
                "UserNotification::send_to_user: FCM v1 push success: status={}, user_pk={}, token_prefix={}, body_snippet={}",
                status,
                user_pk,
                &device.device_token.chars().take(10).collect::<String>(),
                &text.chars().take(100).collect::<String>()
            );
        }

        Ok(())
    }
}
