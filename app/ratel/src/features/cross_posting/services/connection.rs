//! Shared "seal credentials → upsert SocialConnection" helper.
//!
//! Every connect-* controller (Bluesky app-password, LinkedIn OAuth
//! callback, Threads later) ends with the same five steps:
//!   1. AEAD-seal the platform-specific [`DecryptedCredentials`] blob.
//!   2. Look up the user's existing `SocialConnection` row for this
//!      platform — used to preserve `auto_post_enabled`,
//!      `posts_syndicated_count`, and `created_at` across reconnects.
//!   3. Build the new row with `status = Connected`.
//!   4. `upsert` (NOT `create`) so reconnect after Disconnect — which
//!      soft-deletes by flipping `status = Revoked` — overwrites cleanly.
//!   5. Return the persisted row.
//!
//! Per-platform controllers own everything *before* step 1 (input
//! validation, adapter API call, building the typed credentials variant)
//! — that's where the platforms genuinely diverge. The post-validation
//! tail is platform-agnostic and lives here.

use crate::common::*;
use crate::features::cross_posting::models::{ConnectionStatus, SocialConnection};
use crate::features::cross_posting::services::adapters::DecryptedCredentials;
use crate::features::cross_posting::services::credentials::seal_credentials;
use crate::features::cross_posting::types::{CrossPostingError, SocialPlatform};

/// Shape that callers fill in once their platform-specific validation
/// succeeded. Field-level docs match the homonymous `SocialConnection`
/// fields they map onto.
pub struct ConnectionUpsert {
    pub user_pk: Partition,
    pub platform: SocialPlatform,
    /// Decrypted credential blob to seal. Caller is responsible for
    /// constructing the right `DecryptedCredentials::*` variant for the
    /// platform — sealing is opaque, so a Bluesky controller passing a
    /// `LinkedIn` variant would persist successfully but the dispatcher
    /// would fail later with an "unwrap" mismatch. The platform field
    /// above and the variant SHOULD agree; we don't double-check at
    /// runtime to keep the helper allocation-free.
    pub decrypted: DecryptedCredentials,
    /// Display name shown in the connections UI. For Bluesky this is the
    /// `@user.bsky.social` handle; for LinkedIn it's the user's display
    /// name from `/v2/userinfo` (`name` field).
    pub external_handle: String,
    /// Stable platform-side ID — Bluesky DID, LinkedIn member URN, etc.
    /// Used by the dispatcher to build per-post URLs and by reconcile
    /// probes.
    pub external_user_id: String,
    /// Token expiry epoch-millis when the platform issues short-lived
    /// access tokens (LinkedIn, Threads). `None` for Bluesky app-password
    /// sessions, which don't expire until the user revokes the app
    /// password manually.
    pub token_expires_at: Option<i64>,
}

/// Apply steps 1-5 above. On success returns the freshly persisted row;
/// caller typically does `.into()` to convert to `ConnectionResponse` for
/// the HTTP response body.
#[cfg(feature = "server")]
pub async fn seal_and_upsert_connection(
    cli: &aws_sdk_dynamodb::Client,
    upsert: ConnectionUpsert,
) -> Result<SocialConnection> {
    let ConnectionUpsert {
        user_pk,
        platform,
        decrypted,
        external_handle,
        external_user_id,
        token_expires_at,
    } = upsert;

    let ciphertext = seal_credentials(&decrypted).map_err(|e| {
        crate::error!("seal_and_upsert_connection seal failed: {e}");
        CrossPostingError::ConnectFailed
    })?;

    let sk = EntityType::SocialConnection(platform.to_string());
    let now = crate::common::utils::time::now();

    // Reconnect preserves prior counts / created_at so the user's
    // syndication history continues uninterrupted across token-expiry-
    // driven re-auth.
    let existing = SocialConnection::get(cli, user_pk.clone(), Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("seal_and_upsert_connection lookup failed: {e}");
            CrossPostingError::ConnectFailed
        })?;

    let row = SocialConnection {
        pk: user_pk,
        sk,
        platform_status: SocialConnection::platform_status_key(
            platform,
            ConnectionStatus::Connected,
        ),
        platform,
        status: ConnectionStatus::Connected,
        external_handle,
        external_user_id,
        credential_ciphertext: ciphertext,
        token_expires_at,
        auto_post_enabled: existing.as_ref().map_or(true, |c| c.auto_post_enabled),
        posts_syndicated_count: existing.as_ref().map_or(0, |c| c.posts_syndicated_count),
        last_synced_at: existing.as_ref().and_then(|c| c.last_synced_at),
        created_at: existing.as_ref().map_or(now, |c| c.created_at),
        updated_at: now,
    };

    // `upsert` (not `create`) so reconnect after Disconnect — which
    // soft-deletes by flipping `status = Revoked` — overwrites cleanly.
    // `create`'s `attribute_not_exists(pk)` would fail on the second
    // attempt.
    row.upsert(cli).await.map_err(|e| {
        crate::error!("seal_and_upsert_connection persist failed: {e}");
        CrossPostingError::ConnectFailed
    })?;

    Ok(row)
}
