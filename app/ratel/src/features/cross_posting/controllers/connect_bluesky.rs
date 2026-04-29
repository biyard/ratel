use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::models::{ConnectionStatus, SocialConnection};
use crate::features::cross_posting::services::adapters::{BlueskyAdapter, DecryptedCredentials};
use crate::features::cross_posting::services::credentials::seal_credentials;
use crate::features::cross_posting::types::{
    ConnectBlueskyRequest, ConnectionResponse, CrossPostingError, SocialPlatform,
};

#[post("/api/cross-posting/connections/bluesky", user: User)]
pub async fn connect_bluesky_handler(req: ConnectBlueskyRequest) -> Result<ConnectionResponse> {
    let handle = req.handle.trim();
    let app_password = req.app_password.trim();
    if handle.is_empty() || app_password.is_empty() {
        return Err(CrossPostingError::ConnectFailed.into());
    }

    // Validate against Bluesky — exchange app password for a session.
    let adapter = BlueskyAdapter::new();
    let session = adapter.create_session(handle, app_password).await.map_err(|e| {
        crate::error!("connect_bluesky createSession failed: {e}");
        CrossPostingError::BlueskyAuthFailed
    })?;

    // Seal the session (NOT the app password — discarded after this point).
    let decrypted = DecryptedCredentials::Bluesky {
        did: session.did.clone(),
        handle: session.handle.clone(),
        access_jwt: session.access_jwt,
        refresh_jwt: session.refresh_jwt,
    };
    let ciphertext = seal_credentials(&decrypted).map_err(|e| {
        crate::error!("connect_bluesky seal failed: {e}");
        CrossPostingError::ConnectFailed
    })?;

    // Reconnect preserves prior counts / created_at so the user's syndication
    // history continues uninterrupted across token-expiry-driven re-auth.
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk: EntityType = EntityType::SocialConnection(SocialPlatform::Bluesky.to_string());
    let now = crate::common::utils::time::now();

    let existing = SocialConnection::get(cli, user.pk.clone(), Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("connect_bluesky lookup failed: {e}");
            CrossPostingError::ConnectFailed
        })?;

    let row = SocialConnection {
        pk: user.pk.clone(),
        sk: sk.clone(),
        platform_status: format!("{}#{}", SocialPlatform::Bluesky, ConnectionStatus::Connected),
        platform: SocialPlatform::Bluesky,
        status: ConnectionStatus::Connected,
        external_handle: session.handle,
        external_user_id: session.did,
        credential_ciphertext: ciphertext,
        token_expires_at: None, // app-password sessions do not expire
        auto_post_enabled: existing.as_ref().map_or(true, |c| c.auto_post_enabled),
        posts_syndicated_count: existing.as_ref().map_or(0, |c| c.posts_syndicated_count),
        last_synced_at: existing.as_ref().and_then(|c| c.last_synced_at),
        created_at: existing.as_ref().map_or(now, |c| c.created_at),
        updated_at: now,
    };

    row.create(cli).await.map_err(|e| {
        crate::error!("connect_bluesky persist failed: {e}");
        CrossPostingError::ConnectFailed
    })?;

    Ok(row.into())
}
