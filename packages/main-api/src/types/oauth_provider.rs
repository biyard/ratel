use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    DynamoEnum,
    JsonSchema,
    Copy,
)]
pub enum Provider {
    #[default]
    Google,
    // Telegram,
}

impl Provider {
    pub fn oidc_userinfo_url(&self) -> &'static str {
        match self {
            Provider::Google => "https://openidconnect.googleapis.com/v1/userinfo",
        }
    }

    pub async fn get_email(&self, access_token: &str) -> Result<String, crate::Error2> {
        let url = self.oidc_userinfo_url();
        let UserInfo { email } = reqwest::Client::new()
            .get(url)
            .bearer_auth(access_token)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(email)
    }
}

#[derive(Debug, serde::Deserialize)]
struct UserInfo {
    email: String,
}
