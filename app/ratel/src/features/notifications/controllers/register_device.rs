use crate::common::*;
use crate::common::models::notification::UserDeviceToken;
use crate::features::auth::User;
use crate::features::notifications::types::NotificationsError;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// Body for registering (or refreshing) a device's push token.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct RegisterDeviceRequest {
    /// Stable per-install device id (the row key — re-register overwrites).
    pub device_id: String,
    /// FCM (Android) / APNs (iOS) registration token.
    pub token: String,
    /// "android" | "ios" | "web".
    pub platform: String,
}

/// Upsert the caller's push token for one device. Idempotent: re-registering
/// the same `device_id` overwrites the token (FCM rotates tokens) and refreshes
/// the TTL.
#[post("/api/devices", user: User)]
pub async fn register_device_handler(req: RegisterDeviceRequest) -> Result<()> {
    if req.device_id.trim().is_empty() || req.token.trim().is_empty() {
        return Err(NotificationsError::DeviceRegisterFailed.into());
    }

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    UserDeviceToken::new(user.pk.clone(), &req.device_id, req.token, req.platform)
        .create(cli)
        .await
        .map_err(|e| {
            crate::error!("register_device failed: {e}");
            NotificationsError::DeviceRegisterFailed
        })?;

    Ok(())
}
