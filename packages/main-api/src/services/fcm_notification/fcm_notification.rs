use super::*;
use crate::config::GoogleCloudConfig;
use crate::utils::firebase::oauth::get_fcm_access_token;
use crate::*;
use std::collections::HashMap;

#[derive(Serialize)]
struct FcmNotification {
    title: String,
    body: String,
}

#[derive(Serialize)]
struct FcmMessage {
    token: String,
    notification: FcmNotification,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<HashMap<String, String>>,
}

#[derive(Serialize)]
struct FcmRequest {
    message: FcmMessage,
}

pub struct FCMService {
    client: reqwest::Client,
    project_id: String,
    access_token: String,
}

impl FCMService {
    pub async fn new() -> Result<Self> {
        let GoogleCloudConfig {
            project_id,
            enable_fcm,
        } = config::get().google_cloud;

        if !enable_fcm {
            tracing::warn!("FCMService::new: FCM is disabled by config, but service is created.");
        }

        let token = get_fcm_access_token().await?;
        Ok(FCMService {
            client: reqwest::Client::new(),
            project_id: project_id.to_string(),
            access_token: token,
        })
    }

    async fn send_one(
        &self,
        title: &str,
        body: &str,
        token: String,
        data: Option<HashMap<String, String>>,
    ) -> Result<()> {
        let endpoint = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        );

        let req_body = FcmRequest {
            message: FcmMessage {
                token: token.to_string(),
                notification: FcmNotification {
                    title: title.to_string(),
                    body: body.to_string(),
                },
                data,
            },
        };

        let res = self
            .client
            .post(&endpoint)
            .bearer_auth(&self.access_token)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| Error::InternalServerError(format!("FCM v1 request failed: {e}")))?;

        let status = res.status();
        let text = res.text().await.unwrap_or_default();

        if !status.is_success() {
            tracing::error!(
                "FCMService::send_one: push failed: status={}, body={}, token_prefix={}",
                status,
                text,
                &token.chars().take(10).collect::<String>(),
            );
        } else {
            tracing::info!(
                "FCMService::send_one: push success: status={}, token_prefix={}, resp_snippet={}",
                status,
                &token.chars().take(10).collect::<String>(),
                &text.chars().take(100).collect::<String>(),
            );
        }

        Ok(())
    }

    pub async fn send_notification_with_deeplink(
        &mut self,
        title: &str,
        body: &str,
        device_tokens: Vec<String>,
        deeplink: &str,
    ) -> Result<()> {
        let mut data = HashMap::new();
        data.insert("deeplink".to_string(), deeplink.to_string());

        self.send_notification(title, body, device_tokens, Some(data))
            .await
    }

    pub async fn send_notification(
        &mut self,
        title: &str,
        body: &str,
        device_tokens: Vec<String>,
        data: Option<HashMap<String, String>>,
    ) -> Result<()> {
        use futures::future::join_all;

        if device_tokens.is_empty() {
            tracing::warn!("FCMService::send_notification: no device tokens, skip.");
            return Ok(());
        }

        let mut tasks = Vec::with_capacity(device_tokens.len());
        for token in device_tokens {
            let data_clone = data.clone();
            // FIXME: implement failover / retry logic
            tasks.push(self.send_one(title, body, token.clone(), data_clone));
        }

        let results = join_all(tasks).await;

        let mut failed = 0;
        for r in results {
            if r.is_err() {
                failed += 1;
            }
        }

        if failed > 0 {
            tracing::warn!("FCMService::send_notification: {} pushes failed", failed);
        }

        Ok(())
    }
}
