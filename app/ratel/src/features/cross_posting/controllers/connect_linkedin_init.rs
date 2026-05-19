use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::types::{
    CrossPostingError, LinkedInOauthInitRequest, LinkedInOauthInitResponse,
};

/// LinkedIn OAuth scopes Phase 1B needs:
/// - `openid` + `profile` — to call `/v2/userinfo` and read the
///   member's stable `sub` (becomes the `member_urn` in
///   `DecryptedCredentials::LinkedIn`).
/// - `w_member_social` — to call `/v2/ugcPosts` with `author=urn:li:person:{sub}`.
///
/// `email` scope is intentionally NOT requested — Ratel already has the
/// user's email from the sign-in flow, and LinkedIn's app-review process
/// is faster the smaller the requested scope set.
const LINKEDIN_SCOPES: &str = "openid profile w_member_social";

#[cfg(feature = "server")]
const LINKEDIN_AUTHORIZE_HOST: &str = "https://www.linkedin.com";

#[post(
    "/api/cross-posting/connections/linkedin/init",
    user: User
)]
pub async fn connect_linkedin_init_handler(
    req: LinkedInOauthInitRequest,
) -> Result<LinkedInOauthInitResponse> {
    use crate::features::cross_posting::services::oauth_state;

    let client_id = option_env!("LINKEDIN_CLIENT_ID")
        .filter(|v| !v.is_empty())
        .ok_or_else(|| {
            crate::error!("LINKEDIN_CLIENT_ID not configured at compile time");
            CrossPostingError::LinkedInAuthFailed
        })?;

    let state = oauth_state::encode(&user.pk, req.return_to.as_deref()).map_err(|e| {
        crate::error!("connect_linkedin_init state encode failed: {e}");
        CrossPostingError::LinkedInAuthFailed
    })?;

    // Redirect URI must EXACTLY match the value registered under "Authorized
    // redirect URLs" in the LinkedIn Developer Portal — including scheme,
    // host, port, and trailing-slash. We register one URL per Ratel env
    // (local / dev / prod) so the same code path picks the right one
    // automatically via `site_base_url()`.
    let redirect_uri = linkedin_redirect_uri();

    // The `&prompt=consent` would force LinkedIn to show the consent
    // screen even on re-auth — useful for testing, but skipped in
    // production so existing users with valid sessions can reconnect
    // in one click.
    let authorize_url = format!(
        "{host}/oauth/v2/authorization?response_type=code\
         &client_id={cid}\
         &redirect_uri={redir}\
         &scope={scopes}\
         &state={state}",
        host = LINKEDIN_AUTHORIZE_HOST,
        cid = urlencoding::encode(client_id),
        redir = urlencoding::encode(&redirect_uri),
        scopes = urlencoding::encode(LINKEDIN_SCOPES),
        state = urlencoding::encode(&state),
    );

    Ok(LinkedInOauthInitResponse { authorize_url })
}

/// Build the canonical callback URL for the current Ratel environment.
/// Pulled out so the callback controller can reference the same string
/// when calling `LinkedInAdapter::exchange_code` — LinkedIn's token
/// endpoint requires the `redirect_uri` parameter to match the one used
/// during authorization byte-for-byte.
#[cfg(feature = "server")]
pub fn linkedin_redirect_uri() -> String {
    format!(
        "{}/api/cross-posting/connections/linkedin/callback",
        crate::common::config::site_base_url()
    )
}
