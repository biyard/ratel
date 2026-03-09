use std::{fmt::Display, str::FromStr};

use crate::common::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Environment {
    Local,
    Dev,
    Staging,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        let default_env = option_env!("ENV").unwrap_or("local");
        default_env.parse().unwrap_or(Environment::Local)
    }
}

impl FromStr for Environment {
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let e = match s.to_lowercase().as_str() {
            "local" => Environment::Local,
            "dev" | "development" => Environment::Dev,
            "staging" => Environment::Staging,
            "prod" | "production" => Environment::Production,
            _ => {
                warn!("Unrecognized environment '{}', defaulting to Local", s);
                Environment::Local
            }
        };

        Ok(e)
    }

    type Err = String;
}

impl Into<String> for Environment {
    fn into(self) -> String {
        match self {
            Environment::Local => "local".to_string(),
            Environment::Dev => "dev".to_string(),
            Environment::Staging => "stg".to_string(),
            Environment::Production => "prod".to_string(),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = (*self).into();
        write!(f, "{}", s)
    }
}
