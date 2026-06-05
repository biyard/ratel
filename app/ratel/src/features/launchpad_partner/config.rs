//! Config for the Launchpad partner integration.
//!
//! Read at RUNTIME (`std::env::var`) so the shared secret / project id —
//! which Launchpad's admin generates per project at runtime — can be set
//! without recompiling ratel. Falls back to a compile-time `option_env!`
//! value, then a dev default.

#[derive(Debug, Clone)]
pub struct LaunchpadPartnerConfig {
    /// Base URL of the Launchpad app the connect button points at.
    pub base_url: String,
    /// Launchpad project id this ratel instance is registered as.
    pub project_id: String,
    /// Shared secret: AES key material for the user token AND HMAC key for
    /// verifying Launchpad callbacks. MUST equal the Launchpad project's
    /// `company_secret_key`.
    pub shared_secret: String,
    /// Symbol returned to Launchpad in point lookups (cosmetic).
    pub point_symbol: String,
}

fn env_or(name: &str, compile: Option<&str>, default: &str) -> String {
    // Filter empty on BOTH layers so a `LAUNCHPAD_PROJECT_ID=` (empty)
    // env passed during `cargo build` doesn't bake `Some("")` into the
    // binary via `option_env!` and silently defeat the default. The
    // Makefile's BUILD_ENV exports `LAUNCHPAD_*=$(LAUNCHPAD_*)` even
    // when the Make variable is unset, so this guard is what keeps the
    // test fixtures' default secret / project id usable in CI.
    std::env::var(name)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .or_else(|| {
            compile
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| default.to_string())
}

impl Default for LaunchpadPartnerConfig {
    fn default() -> Self {
        Self {
            base_url: env_or(
                "LAUNCHPAD_BASE_URL",
                option_env!("LAUNCHPAD_BASE_URL"),
                "http://localhost:8080",
            ),
            project_id: env_or(
                "LAUNCHPAD_PROJECT_ID",
                option_env!("LAUNCHPAD_PROJECT_ID"),
                "launchpad-demo",
            ),
            shared_secret: env_or(
                "LAUNCHPAD_PARTNER_SECRET",
                option_env!("LAUNCHPAD_PARTNER_SECRET"),
                "dev-demo-shared-secret-change-me",
            ),
            point_symbol: env_or(
                "LAUNCHPAD_POINT_SYMBOL",
                option_env!("LAUNCHPAD_POINT_SYMBOL"),
                "P",
            ),
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
    }
}
