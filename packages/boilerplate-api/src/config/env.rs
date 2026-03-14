#[derive(Debug, Clone, Copy)]
pub enum Env {
    Local,
    Development,
    Staging,
    Production,
}

impl Default for Env {
    fn default() -> Self {
        let env = option_env!("ENV").unwrap_or("local");

        match env {
            "local" => Env::Local,
            "dev" | "development" => Env::Development,
            "stg" | "staging" => Env::Staging,
            "prod" | "production" => Env::Production,
            _ => panic!("Invalid ENV value"),
        }
    }
}
