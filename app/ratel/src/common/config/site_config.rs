use crate::common::config::Environment;

pub fn site_base_url() -> &'static str {
    match Environment::default() {
        Environment::Local => "http://localhost:8080",
        Environment::Dev => "https://dev.ratel.foundation",
        Environment::Staging => "https://stg.ratel.foundation",
        Environment::Production => "https://ratel.foundation",
    }
}
