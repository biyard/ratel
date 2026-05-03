use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::models::{ConnectionStatus, SocialConnection};
use crate::features::cross_posting::types::{CrossPostingError, SocialPlatform};

/// Soft-delete: zero the credential ciphertext, mark Revoked. The row is
/// kept (not hard-deleted) so historical "posted via …" rendering on past
/// `SyndicationJob` references can still resolve the platform handle.
/// Future cross-posts to this platform stop because Stage 1's
/// `enabled_platforms ∩ connected` check filters out non-Connected rows.
#[delete("/api/cross-posting/connections/{platform}", user: User)]
pub async fn disconnect_handler(platform: SocialPlatform) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk = EntityType::SocialConnection(platform.to_string());
    let now = crate::common::utils::time::now();

    // Load to confirm existence; idempotent — second DELETE on an
    // already-revoked row is a no-op success.
    let existing = SocialConnection::get(cli, user.pk.clone(), Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("disconnect lookup failed: {e}");
            CrossPostingError::UpdateFailed
        })?;

    let Some(_existing) = existing else {
        return Ok(());
    };

    SocialConnection::updater(user.pk.clone(), sk)
        .with_status(ConnectionStatus::Revoked)
        .with_credential_ciphertext(Vec::new())
        .with_platform_status(SocialConnection::platform_status_key(
            platform,
            ConnectionStatus::Revoked,
        ))
        .with_updated_at(now)
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("disconnect update failed: {e}");
            CrossPostingError::UpdateFailed
        })?;

    Ok(())
}
