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
    pub fn token_info(&self, id_token: &str) -> String {
        match self {
            Provider::Google => {
                format!("https://oauth2.googleapis.com/tokeninfo?id_token={id_token}")
            }
        }
    }
}
