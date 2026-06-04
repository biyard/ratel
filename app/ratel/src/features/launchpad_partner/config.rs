//! Env-backed config for the Launchpad partner integration.
//! Mirrors `common/config/server/biyard.rs`: compile-time `option_env!`
//! for secrets/ids, runtime `std::env::var` for the URL.

#[derive(Debug, Clone)]
pub struct LaunchpadPartnerConfig {
    /// Base URL of the Launchpad app the connect button points at.
    pub base_url: String,
    /// Launchpad project id this ratel instance is registered as.
    pub project_id: &'static str,
    /// Shared secret: AES key material for the user token AND HMAC key
    /// for verifying Launchpad callbacks. Must match Launchpad's project
    /// company_secret_key.
    pub shared_secret: &'static str,
    /// Symbol returned to Launchpad in point lookups (cosmetic).
    pub point_symbol: &'static str,
}

impl Default for LaunchpadPartnerConfig {
    fn default() -> Self {
        Self {
            base_url: std::env::var("LAUNCHPAD_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            project_id: option_env!("LAUNCHPAD_PROJECT_ID").unwrap_or("launchpad-demo"),
            shared_secret: option_env!("LAUNCHPAD_PARTNER_SECRET")
                .unwrap_or("dev-demo-shared-secret-change-me"),
            point_symbol: option_env!("LAUNCHPAD_POINT_SYMBOL").unwrap_or("P"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_present() {
        let cfg = LaunchpadPartnerConfig::default();
        assert!(!cfg.base_url.is_empty());
        assert!(!cfg.project_id.is_empty());
        assert!(!cfg.shared_secret.is_empty());
        assert_eq!(cfg.point_symbol, "P");
    }
}
