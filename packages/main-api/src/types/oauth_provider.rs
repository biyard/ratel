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
}
