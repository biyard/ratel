use common::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    Copy,
)]
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

    pub async fn get_email(&self, access_token: &str) -> Result<String> {
        let url = self.oidc_userinfo_url();
        let UserInfo { email } = reqwest::Client::new()
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| Error::InternalServerError(format!("OAuth request failed: {}", e)))?
            .error_for_status()
            .map_err(|e| Error::InternalServerError(format!("OAuth error status: {}", e)))?
            .json()
            .await
            .map_err(|e| Error::InternalServerError(format!("OAuth parse failed: {}", e)))?;

        Ok(email)
    }
}

#[derive(Debug, serde::Deserialize)]
struct UserInfo {
    email: String,
}
