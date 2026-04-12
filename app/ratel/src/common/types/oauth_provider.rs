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
impl OauthProvider {
    pub async fn get_email(&self, access_token: &str) -> crate::common::Result<String> {
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
}
