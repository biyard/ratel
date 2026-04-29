use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::models::SocialConnection;
use crate::features::cross_posting::types::{
    ConnectionResponse, CrossPostingError, SocialPlatform, ToggleAutoPostRequest,
};
use std::str::FromStr;

#[patch("/api/cross-posting/connections/{platform}", user: User)]
pub async fn toggle_auto_post_handler(
    platform: String,
    req: ToggleAutoPostRequest,
) -> Result<ConnectionResponse> {
    let platform: SocialPlatform = SocialPlatform::from_str(&platform)
        .map_err(|_| CrossPostingError::ConnectionNotFound)?;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let sk: EntityType = EntityType::SocialConnection(platform.to_string());
    let now = crate::common::utils::time::now();

    let existing = SocialConnection::get(cli, user.pk.clone(), Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("toggle_auto_post lookup failed: {e}");
            CrossPostingError::UpdateFailed
        })?
        .ok_or(CrossPostingError::ConnectionNotFound)?;

    SocialConnection::updater(user.pk.clone(), sk)
        .with_auto_post_enabled(req.auto_post_enabled)
        .with_updated_at(now)
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("toggle_auto_post update failed: {e}");
            CrossPostingError::UpdateFailed
        })?;

    let mut updated = existing;
    updated.auto_post_enabled = req.auto_post_enabled;
    updated.updated_at = now;
    Ok(updated.into())
}
