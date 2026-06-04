use crate::common::config::Environment;

pub fn site_base_url() -> &'static str {
    match Environment::default() {
        // `make run` serves ratel on :8000 (`dx serve --port 8000`); :8080 is
        // launchpad's local port. The Launchpad redirect_uri is built from this,
        // so it must point at where ratel actually serves.
        Environment::Local => "http://localhost:8000",
        Environment::Dev => "https://dev.ratel.foundation",
        Environment::Staging => "https://stg.ratel.foundation",
        Environment::Production => "https://ratel.foundation",
    }
}
