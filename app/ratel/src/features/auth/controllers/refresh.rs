//! Refresh-token-based session restore.
//!
//! Mobile clients can't keep the `tower_sessions` cookie across app
//! restarts (the in-process reqwest cookie jar is volatile). After login
//! they save the issued refresh token + their stable device id to the
//! WebView's `localStorage`, and call `/api/auth/refresh` on next launch
//! to mint a fresh server session before any authenticated calls fire.
//!
//! On every successful refresh we rotate the token (issue a new one,
//! invalidate the old) so a stolen token can't be replayed indefinitely.

#[cfg(feature = "server")]
use crate::common::utils::sha256::sha256_base64url;
use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RefreshSessionRequest {
    pub refresh_token: String,
    pub device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RefreshSessionResponse {
    #[serde(flatten)]
    pub user: User,
    /// Rotated refresh token. Mobile must overwrite the saved value with
    /// this on every successful refresh.
    pub refresh_token: String,
}

#[post("/api/auth/refresh", session: Extension<tower_sessions::Session>)]
pub async fn refresh_session_handler(
    req: RefreshSessionRequest,
) -> Result<RefreshSessionResponse> {
    let conf = crate::features::auth::config::get();
    let cli = conf.dynamodb();
    let Extension(session) = session;

    let RefreshSessionRequest {
        refresh_token,
        device_id,
    } = req;

    // Look up by device_id (gsi1) — there should be at most one active row.
    let (rows, _) = UserRefreshToken::find_by_device_id(
        cli,
        &device_id,
        UserRefreshTokenQueryOption::builder().limit(1),
    )
    .await
    .map_err(|e| {
        crate::error!("refresh: device_id lookup failed: {e}");
        Error::NoSessionFound
    })?;

    let row = rows.into_iter().next().ok_or(Error::NoSessionFound)?;

    if row.revoked {
        return Err(Error::NoSessionFound);
    }
    if let Some(exp) = row.expired_at {
        if exp < crate::common::utils::time::now() {
            return Err(Error::NoSessionFound);
        }
    }

    // Constant-time compare via fixed-length base64 sha256 strings.
    let presented_hash = sha256_base64url(&refresh_token);
    if presented_hash != row.token_hash {
        return Err(Error::NoSessionFound);
    }

    let user = User::get(cli, row.pk.clone(), Some(EntityType::User))
        .await
        .map_err(|e| {
            crate::error!("refresh: user lookup failed: {e}");
            Error::NoSessionFound
        })?
        .ok_or(Error::NoSessionFound)?;

    // Mint a fresh session cookie for this client.
    session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await?;

    // Rotate the refresh token — the previous hash is overwritten in place
    // because `device_id` keys the row.
    let (rotated, plain) = UserRefreshToken::new(&user, device_id);
    rotated.upsert(cli).await?;

    Ok(RefreshSessionResponse {
        user,
        refresh_token: plain,
    })
}
