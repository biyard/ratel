use std::fmt::Display;

use crate::*;

#[derive(Clone, Debug, Copy, Default)]
pub enum Environment {
    #[default]
    Local,
    Dev,
    Staging,
    Production,
}

impl Environment {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "local" => Environment::Local,
            "dev" | "development" => Environment::Dev,
            "staging" => Environment::Staging,
            "prod" | "production" => Environment::Production,
            _ => {
                warn!("Unrecognized environment '{}', defaulting to Local", s);
                Environment::Local
            } // default to Local if unrecognized
        }
    }
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
