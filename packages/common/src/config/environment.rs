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
