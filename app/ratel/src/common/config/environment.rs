use std::{fmt::Display, str::FromStr};

use crate::common::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Environment {
    Local,
    Dev,
    Staging,
    Production,
}

impl Environment {
    pub fn mobile_endpoint(self) -> &'static str {
        match self {
            // Local builds for Android talk to the dev server running on the
            // developer's workstation, so the device (emulator or physical
            // phone on the same Wi-Fi) needs a reachable LAN address — not
            // `localhost`, which would resolve to the device itself.
            //
            // `MOBILE_API_URL` is injected by `make android` (see Makefile —
            // it auto-detects the host's LAN IP via `ip route get`). The
            // `10.0.2.2` fallback is the Android emulator's alias for the
            // host loopback, so raw `cargo build` still produces a working
            // emulator binary when the Makefile isn't in play.
            Environment::Local => option_env!("MOBILE_API_URL").unwrap_or("http://10.0.2.2:8080"),
            Environment::Dev => "https://dev.ratel.foundation",
            Environment::Staging => "https://stg.ratel.foundation",
            Environment::Production => "https://ratel.foundation",
        }
    }

    pub fn web_endpoint(self) -> &'static str {
        match self {
            Environment::Local => "http://localhost:8080",
            Environment::Dev => "https://dev.ratel.foundation",
            Environment::Staging => "https://stg.ratel.foundation",
            Environment::Production => "https://ratel.foundation",
        }
    }
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
