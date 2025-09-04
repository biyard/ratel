use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
}

impl GrantType {
    pub fn as_str(&self) -> &'static str {
        match self {
            GrantType::AuthorizationCode => "authorization_code",
            GrantType::RefreshToken => "refresh_token",
        }
    }
}

impl std::str::FromStr for GrantType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "authorization_code" => Ok(GrantType::AuthorizationCode),
            "refresh_token" => Ok(GrantType::RefreshToken),
            _ => Err(format!("Invalid grant type: {}", s)),
        }
    }
}
