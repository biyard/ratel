use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// Push tokens expire if a device is idle; refreshed on every register call.
pub const DEVICE_TOKEN_TTL_DAYS: i64 = 60;

/// A registered push-notification token for one user device. Looked up by
/// `pk = User(user_id)` (a base-table query with `begins_with` on the sk) when
/// fanning a notification out to all of a user's devices. Keyed by a stable
/// per-install `device_id` so re-registering the same device overwrites its
/// token (FCM rotates tokens) instead of piling up duplicates.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UserDeviceToken {
    pub pk: Partition, // User(user_id)

    pub sk: EntityType, // UserDeviceToken(device_id)

    /// FCM (Android) / APNs (iOS) registration token.
    pub token: String,

    /// "android" | "ios" | "web".
    pub platform: String,

    pub created_at: i64,
    pub updated_at: i64,

    /// DynamoDB TTL field (epoch seconds).
    pub expires_at: i64,
}

#[cfg(feature = "server")]
impl UserDeviceToken {
    pub fn new(recipient_pk: Partition, device_id: &str, token: String, platform: String) -> Self {
        let now_ms = crate::common::utils::time::get_now_timestamp_millis();
        let expires_at = (now_ms / 1000) + DEVICE_TOKEN_TTL_DAYS * 86_400;
        Self {
            pk: recipient_pk,
            sk: EntityType::UserDeviceToken(device_id.to_string()),
            token,
            platform,
            created_at: now_ms,
            updated_at: now_ms,
            expires_at,
        }
    }
}
