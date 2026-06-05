//! OAuth-style return hand-back from Launchpad.
//!
//! After a point conversion completes, Launchpad redirects the user back to
//! `/launchpad/return?...` with the receipt fields and an HMAC signature. This
//! server function re-derives the canonical string Launchpad signed and
//! verifies it with the shared secret before the landing page trusts anything.
//!
//! Canonical (must byte-match Launchpad's `build_external_return_url`): the raw
//! (URL-decoded) field values joined by `\n` in this exact order —
//! project_id, conversion_id, brand_tx_id, deducted_points,
//! remaining_points ("" when absent), round_id, community_url.
//! Signature: `HMAC_SHA256(secret, "{ts}.{canonical}")`.

use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Raw return params as received on the redirect URL (already URL-decoded by
/// the router). Numbers stay as strings so the canonical re-derivation is
/// byte-identical to what Launchpad signed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct LaunchpadReturnRequest {
    pub project_id: String,
    pub conversion_id: String,
    pub brand_tx_id: String,
    pub deducted_points: String,
    pub remaining_points: String,
    pub round_id: String,
    pub community_url: String,
    pub ts: String,
    pub sig: String,
}

/// Verified, typed view the landing page renders.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct LaunchpadReturnView {
    /// Signature + freshness both passed. The page must treat a `false` here
    /// as an untrusted/expired hand-back and show an error instead of a link.
    pub verified: bool,
    pub community_url: String,
    pub conversion_id: String,
    pub brand_tx_id: String,
    pub deducted_points: i64,
    pub remaining_points: Option<i64>,
    pub round_id: String,
    pub project_id: String,
}

#[post("/api/launchpad/verify-return")]
pub async fn launchpad_verify_return_handler(
    req: LaunchpadReturnRequest,
) -> crate::common::Result<LaunchpadReturnView> {
    server_impl::verify(req).await
}

#[cfg(feature = "server")]
mod server_impl {
    use super::{LaunchpadReturnRequest, LaunchpadReturnView};
    use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
    use crate::features::launchpad_partner::crypto::verify_signature;

    /// Accept a hand-back signed at most this long ago. Defends against
    /// replay of an old (e.g. shared/bookmarked) return URL.
    const MAX_AGE_MS: i64 = 10 * 60 * 1000;

    pub async fn verify(
        req: LaunchpadReturnRequest,
    ) -> crate::common::Result<LaunchpadReturnView> {
        let cfg = LaunchpadPartnerConfig::default();

        let canonical = [
            req.project_id.as_str(),
            req.conversion_id.as_str(),
            req.brand_tx_id.as_str(),
            req.deducted_points.as_str(),
            req.remaining_points.as_str(),
            req.round_id.as_str(),
            req.community_url.as_str(),
        ]
        .join("\n");

        let sig_ok = verify_signature(&cfg.shared_secret, &req.ts, &req.sig, &canonical);
        if !sig_ok {
            tracing::warn!(
                "launchpad return signature mismatch (project_id={}); check LAUNCHPAD_PARTNER_SECRET matches the launchpad project secret",
                cfg.project_id,
            );
        }

        let fresh = req
            .ts
            .parse::<i64>()
            .ok()
            .map(|ts| {
                let now = crate::common::utils::time::get_now_timestamp_millis();
                let age = now - ts;
                (-MAX_AGE_MS..=MAX_AGE_MS).contains(&age)
            })
            .unwrap_or(false);

        let verified = sig_ok && fresh;

        Ok(LaunchpadReturnView {
            verified,
            community_url: if verified {
                req.community_url
            } else {
                String::new()
            },
            conversion_id: req.conversion_id,
            brand_tx_id: req.brand_tx_id,
            deducted_points: req.deducted_points.parse::<i64>().unwrap_or_default(),
            remaining_points: req.remaining_points.parse::<i64>().ok(),
            round_id: req.round_id,
            project_id: req.project_id,
        })
    }
}
