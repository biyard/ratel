//! Server function that builds the per-user Launchpad handoff URL.
//!
//! Computed server-side (token encryption needs the shared secret) and
//! consumed by the connect button via `use_loader`, so SSR and the
//! hydrated client render the identical token-bearing href.

use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct LaunchpadEntryUrlResponse {
    /// `{base}/connect?project_id={pid}&lp_user={token}`
    pub url: String,
}

#[get("/api/launchpad/entry-url", user: crate::features::auth::User)]
pub async fn launchpad_entry_url_handler() -> crate::common::Result<LaunchpadEntryUrlResponse> {
    use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
    use crate::features::launchpad_partner::crypto::encrypt_user_token;

    let cfg = LaunchpadPartnerConfig::default();
    let base = cfg.base_url.trim_end_matches('/');
    let token = encrypt_user_token(&cfg.shared_secret, &user.id()).map_err(|e| {
        crate::error!("launchpad entry-url: token encryption failed: {e}");
        crate::common::Error::Internal
    })?;
    // OAuth-style return target: Launchpad bounces the user back here (signed)
    // after the conversion completes. Must live on a host Launchpad allowlists
    // against the project's brand_page_url / callback_base_url.
    let return_url = format!(
        "{}/launchpad/return",
        site_base_url().trim_end_matches('/')
    );
    let url = format!(
        "{base}/connect?project_id={}&lp_user={token}&redirect_uri={}",
        cfg.project_id,
        urlencoding::encode(&return_url),
    );

    Ok(LaunchpadEntryUrlResponse { url })
}
