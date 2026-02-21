use crate::{DeserializeFromStr, DynamoEnum, SerializeDisplay};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum Provider {
    #[default]
    Google,
}

impl Provider {
    pub fn oidc_userinfo_url(&self) -> &'static str {
        match self {
            Provider::Google => "https://openidconnect.googleapis.com/v1/userinfo",
        }
    }
}

#[cfg(feature = "server")]
#[derive(Debug, serde::Deserialize)]
struct UserInfo {
    email: String,
}

#[cfg(feature = "server")]
impl Provider {
    pub async fn get_email(&self, access_token: &str) -> crate::Result<String> {
        let url = self.oidc_userinfo_url();
        let UserInfo { email } = reqwest::Client::new()
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| crate::Error::InternalServerError(format!("OAuth request failed: {}", e)))?
            .error_for_status()
            .map_err(|e| crate::Error::InternalServerError(format!("OAuth request failed: {}", e)))?
            .json()
            .await
            .map_err(|e| crate::Error::InternalServerError(format!("OAuth response parse failed: {}", e)))?;

        Ok(email)
    }
}
