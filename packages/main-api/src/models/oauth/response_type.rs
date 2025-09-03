use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Code,
}

impl ResponseType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResponseType::Code => "code",
        }
    }

    // Listing all possible values for validation purposes
    pub fn variants() -> Vec<String> {
        vec![ResponseType::Code.as_str().to_string()]
    }
}

impl std::str::FromStr for ResponseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "code" => Ok(ResponseType::Code),
            _ => Err(format!("Invalid response type: {}", s)),
        }
    }
}
