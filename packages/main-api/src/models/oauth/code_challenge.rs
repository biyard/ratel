use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, JsonSchema)]
#[serde(rename = "refresh_token")]
pub enum CodeChallengeMethod {
    S256,
}
impl CodeChallengeMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            CodeChallengeMethod::S256 => "S256",
        }
    }

    // Listing all possible values for validation purposes
    pub fn variants() -> Vec<String> {
        vec![CodeChallengeMethod::S256.as_str().to_string()]
    }
}

impl std::str::FromStr for CodeChallengeMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "s256" => Ok(CodeChallengeMethod::S256),
            _ => Err(format!("Invalid code challenge method: {}", s)),
        }
    }
}
