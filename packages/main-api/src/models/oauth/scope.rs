use bdk::prelude::*;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Profile,
    Email,
}

impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scope::Profile => "profile",
            Scope::Email => "email",
        }
    }

    // Listing all possible values for validation purposes
    pub fn variants() -> Vec<String> {
        vec![
            Scope::Profile.as_str().to_string(),
            Scope::Email.as_str().to_string(),
        ]
    }
}

impl std::str::FromStr for Scope {
    type Err = ();

    fn from_str(input: &str) -> std::result::Result<Scope, Self::Err> {
        match input.to_lowercase().as_str() {
            "profile" => Ok(Scope::Profile),
            "email" => Ok(Scope::Email),
            _ => Err(()),
        }
    }
}

pub fn deserialize_scope_vec<'de, D>(deserializer: D) -> Result<Vec<Scope>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect())
}
