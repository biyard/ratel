use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::types::{
    ConnectBlueskyRequest, ConnectionResponse, CrossPostingError, SocialPlatform,
};

// `services` is server-only (it pulls reqwest + aws-sdk-* etc.), so the
// imports must be cfg-gated. The handler body is server-only via the
// `#[post(...)]` macro, which generates a thin client-side stub for web
// builds — the body never reaches the web compile path.
#[cfg(feature = "server")]
use crate::features::cross_posting::services::{
    adapters::{BlueskyAdapter, DecryptedCredentials},
    connection::{ConnectionUpsert, seal_and_upsert_connection},
};

// Routed under `/connections/bluesky/connect` rather than the bare
// `/connections/bluesky` so the literal segment doesn't shadow the
// `{platform}` param routes that toggle / disconnect register on the
// same prefix (axum picks literal > param, which would force PATCH and
// DELETE on `bluesky` into a 405).
#[post("/api/cross-posting/connections/bluesky/connect", user: User)]
pub async fn connect_bluesky_handler(req: ConnectBlueskyRequest) -> Result<ConnectionResponse> {
    let handle = req.handle.trim();
    let app_password = req.app_password.trim();
    if handle.is_empty() || app_password.is_empty() {
        return Err(CrossPostingError::ConnectFailed.into());
    }

    // Validate against Bluesky — exchange app password for a session.
    let adapter = BlueskyAdapter::new();
    let session = adapter
        .create_session(handle, app_password)
        .await
        .map_err(|e| {
            crate::error!("connect_bluesky createSession failed: {e}");
            CrossPostingError::BlueskyAuthFailed
        })?;

    // Seal the session (NOT the app password — discarded after this point)
    // and upsert the SocialConnection row via the shared helper. App-
    // password sessions don't expire on Bluesky's side, so no
    // token_expires_at.
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let row = seal_and_upsert_connection(
        cli,
        ConnectionUpsert {
            user_pk: user.pk.clone(),
            platform: SocialPlatform::Bluesky,
            decrypted: DecryptedCredentials::Bluesky {
                did: session.did.clone(),
                handle: session.handle.clone(),
                access_jwt: session.access_jwt,
                refresh_jwt: session.refresh_jwt,
            },
            external_handle: session.handle,
            external_user_id: session.did,
            token_expires_at: None,
        },
    )
    .await?;

    Ok(row.into())
}
