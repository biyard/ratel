use crate::common::{DeserializeFromStr, DynamoEnum, SerializeDisplay};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum OauthProvider {
    #[default]
    Google,
}

impl OauthProvider {
    pub fn oidc_userinfo_url(&self) -> &'static str {
        match self {
            OauthProvider::Google => "https://openidconnect.googleapis.com/v1/userinfo",
        }
    }
}

#[cfg(feature = "server")]
#[derive(Debug, serde::Deserialize)]
struct UserInfo {
    email: String,
}

#[cfg(feature = "server")]
#[derive(Debug, serde::Deserialize)]
struct TokenInfo {
    email: String,
}

#[cfg(feature = "server")]
impl OauthProvider {
    /// Extract the verified email address from a Google OAuth token.
    ///
    /// The wire field is historically called `access_token`, but it actually
    /// carries one of two shapes depending on where the caller signed in:
    ///
    /// - **Web (`GoogleAuthProvider.signInWithPopup` via Firebase)** — a
    ///   real Google **OAuth access token** (opaque, not a JWT). Validated
    ///   against `https://openidconnect.googleapis.com/v1/userinfo`.
    /// - **Android (Credential Manager via `GoogleIdTokenCredential`)** —
    ///   a Google **ID token** (JWT with three `.`-separated segments).
    ///   Validated against `https://oauth2.googleapis.com/tokeninfo?id_token=`,
    ///   which verifies the signature, `aud`, `exp`, and issuer, then
    ///   returns the token's claims.
    ///
    /// We auto-detect by shape: JWTs always have exactly two dots (header,
    /// payload, signature). Anything else is treated as an access token.
    pub async fn get_email(&self, token: &str) -> crate::common::Result<String> {
        if is_jwt(token) {
            self.get_email_from_id_token(token).await
        } else {
            self.get_email_from_access_token(token).await
        }
    }

    async fn get_email_from_access_token(&self, access_token: &str) -> crate::common::Result<String> {
        let url = self.oidc_userinfo_url();
        let UserInfo { email } = reqwest::Client::new()
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| {
                crate::error!("OAuth: {e}");
                crate::common::services::ServiceError::OAuthRequestFailed
            })?
            .error_for_status()
            .map_err(|e| {
                crate::error!("OAuth: {e}");
                crate::common::services::ServiceError::OAuthRequestFailed
            })?
            .json()
            .await
            .map_err(|e| {
                crate::error!("OAuth parse: {e}");
                crate::common::services::ServiceError::OAuthParseFailed
            })?;

        Ok(email)
    }

    async fn get_email_from_id_token(&self, id_token: &str) -> crate::common::Result<String> {
        let TokenInfo { email } = reqwest::Client::new()
            .get("https://oauth2.googleapis.com/tokeninfo")
            .query(&[("id_token", id_token)])
            .send()
            .await
            .map_err(|e| {
                crate::error!("OAuth tokeninfo request: {e}");
                crate::common::services::ServiceError::OAuthRequestFailed
            })?
            .error_for_status()
            .map_err(|e| {
                crate::error!("OAuth tokeninfo status: {e}");
                crate::common::services::ServiceError::OAuthRequestFailed
            })?
            .json()
            .await
            .map_err(|e| {
                crate::error!("OAuth tokeninfo parse: {e}");
                crate::common::services::ServiceError::OAuthParseFailed
            })?;

        Ok(email)
    }
}

/// Cheap JWT detector: Google ID tokens have exactly three non-empty,
/// base64url-ish segments separated by `.`. Google access tokens are
/// opaque strings that start with `ya29.` but may contain zero or one `.`.
#[cfg(feature = "server")]
fn is_jwt(token: &str) -> bool {
    let mut parts = token.splitn(4, '.');
    let header = parts.next().unwrap_or("");
    let payload = parts.next().unwrap_or("");
    let signature = parts.next().unwrap_or("");
    let extra = parts.next();
    extra.is_none()
        && !header.is_empty()
        && !payload.is_empty()
        && !signature.is_empty()
}
