use futures::future::join_all;

use crate::{
    services::fcm_notification::FCMService,
    utils::{aws::DynamoClient, time::get_now_timestamp_millis},
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
    pub device_token: String,

    pub platform: NotificationPlatform,
    pub created_at: i64,

    #[dynamo(index = "gsi2", sk)]
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
    pub async fn device_tokens_by_user(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> Result<Vec<String>> {
        let opt = Self::opt_all().scan_index_forward(false);
        let (items, _) = Self::find_user_notifications_by_user(cli, user_pk, opt).await?;

        Ok(items.into_iter().map(|d| d.device_token).collect())
    }

    pub async fn device_tokens_for_users(
        cli: &aws_sdk_dynamodb::Client,
        user_pks: &[Partition],
    ) -> Result<Vec<String>> {
        if user_pks.is_empty() {
            return Ok(vec![]);
        }

        let tasks = user_pks
            .iter()
            .cloned()
            .map(|pk| async move { Self::device_tokens_by_user(cli, &pk).await });

        let results = join_all(tasks).await;

        let mut all_tokens = Vec::new();
        for res in results {
            let mut tokens = res?;
            all_tokens.append(&mut tokens);
        }

        all_tokens.sort();
        all_tokens.dedup();

        Ok(all_tokens)
    }

    pub async fn send_to_users(
        dynamo: &DynamoClient,
        fcm: &mut FCMService,
        user_pks: &[Partition],
        title: impl Into<String>,
        body: impl Into<String>,
        deeplink: Option<String>,
    ) -> Result<()> {
        let title = title.into();
        let body = body.into();

        let fcm_enabled = config::get().google_cloud.enable_fcm;
        if !fcm_enabled {
            tracing::info!(
                "UserNotification::send_to_users: FCM_ENABLED != true, skip push (user_count={})",
                user_pks.len()
            );
            return Ok(());
        }

        if user_pks.is_empty() {
            tracing::debug!("UserNotification::send_to_users: empty user_pks, skip");
            return Ok(());
        }

        let tokens = UserNotification::device_tokens_for_users(&dynamo.client, user_pks).await?;

        if tokens.is_empty() {
            tracing::debug!(
                "UserNotification::send_to_users: no active device tokens (user_count={})",
                user_pks.len()
            );
            return Ok(());
        }

        if deeplink.is_none() {
            fcm.send_notification(&title, &body, tokens, None).await
        } else {
            fcm.send_notification_with_deeplink(
                &title,
                &body,
                tokens,
                deeplink.unwrap_or_default().as_str(),
            )
            .await
        }
    }

    pub async fn send_to_user(
        dynamo: &DynamoClient,
        fcm: &mut FCMService,
        user_pk: &Partition,
        title: impl Into<String>,
        body: impl Into<String>,
        deeplink: Option<String>,
    ) -> Result<()> {
        Self::send_to_users(
            dynamo,
            fcm,
            std::slice::from_ref(user_pk),
            title,
            body,
            deeplink,
        )
        .await
    }
}
