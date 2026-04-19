//! Stable per-install device id used to scope refresh tokens.
//!
//! On the very first call we mint a UUIDv7 and persist it to the
//! WebView's `localStorage`. Every subsequent call returns the same id,
//! so a refresh token issued for this device on Monday can still be
//! exchanged for a fresh session on Friday after several app restarts.
//!
//! Cross-platform: works on web (browser localStorage), mobile
//! (WebView localStorage that survives Android app restarts), and
//! desktop. On the server (SSR) we never need a device id, so the helper
//! is gated to non-server targets.

use crate::common::utils::storage;

const DEVICE_ID_STORAGE_KEY: &str = "device_id";

/// Get or create the persistent device id. Returns `None` only if storage
/// is genuinely unavailable (private mode, SSR, etc.) — otherwise a fresh
/// UUIDv7 is minted and saved before being returned.
pub async fn get_or_create() -> Option<String> {
    if let Some(existing) = storage::load(DEVICE_ID_STORAGE_KEY).await {
        if !existing.trim().is_empty() {
            return Some(existing);
        }
    }
    let new_id = uuid::Uuid::now_v7().to_string();
    storage::save(DEVICE_ID_STORAGE_KEY, &new_id);
    Some(new_id)
}
