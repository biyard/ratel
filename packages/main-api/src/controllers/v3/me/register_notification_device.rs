use crate::models::NotificationPlatform;
use crate::models::UserNotification;
use crate::*;
use crate::{AppState, models::user::User, *};
use aide::NoApi;
use axum::{Json, extract::State};

#[derive(
    serde::Deserialize, serde::Serialize, Debug, Clone, schemars::JsonSchema, aide::OperationIo,
)]
pub struct RegisterNotificationDeviceRequest {
    pub device_token: String,
    pub platform: NotificationPlatform,
    pub device_id: String,
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    Debug,
    Clone,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct RegisterNotificationDeviceResponse {
    pub created: bool,
}

pub async fn register_notification_device_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<RegisterNotificationDeviceRequest>,
) -> Result<Json<RegisterNotificationDeviceResponse>> {
    if req.device_token.trim().is_empty() {
        return Err(Error::BadRequest("device_token is required".to_string()));
    }

    if req.device_id.trim().is_empty() {
        return Err(Error::BadRequest("device_id is required".to_string()));
    }

    let sk = EntityType::UserNotification(req.device_id.clone());

    let existing = UserNotification::get(&dynamo.client, user.pk.clone(), Some(sk.clone())).await?;

    if let Some(mut n) = existing {
        n.device_token = req.device_token.clone();
        n.platform = req.platform;
        n.touch();

        UserNotification::updater(user.pk, sk)
            .with_device_token(n.device_token.clone())
            .with_platform(n.platform)
            .with_updated_at(n.updated_at)
            .with_last_used_at(n.last_used_at.unwrap_or_default())
            .execute(&dynamo.client)
            .await?;

        Ok(Json(RegisterNotificationDeviceResponse { created: false }))
    } else {
        let mut n = UserNotification::new(
            user.pk.clone(),
            req.device_token.clone(),
            req.platform,
            req.device_id.clone(),
        );
        n.touch();

        n.create(&dynamo.client).await?;

        Ok(Json(RegisterNotificationDeviceResponse { created: true }))
    }
}
