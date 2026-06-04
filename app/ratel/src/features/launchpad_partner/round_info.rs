//! Current launchpad round info (scope-A). Queries launchpad's public
//! `/rounds` endpoint and surfaces the open (or most recent) round so the
//! rewards hero can show "my points ÷ round total = share of pool".

use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct LaunchpadRoundInfo {
    pub has_round: bool,
    pub name: String,
    /// "draft" | "open" | "closed" | "distributed"
    pub status: String,
    pub total_points_registered: i64,
    pub total_entries: i64,
    pub opens_at: i64,
    pub closes_at: i64,
}

#[get("/api/launchpad/round-info")]
pub async fn launchpad_round_info_handler() -> crate::common::Result<LaunchpadRoundInfo> {
    server_impl::fetch().await
}

#[cfg(feature = "server")]
mod server_impl {
    use super::LaunchpadRoundInfo;
    use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;

    pub async fn fetch() -> crate::common::Result<LaunchpadRoundInfo> {
        let cfg = LaunchpadPartnerConfig::default();
        let url = format!(
            "{}/api/onchain/projects/{}/rounds",
            cfg.base_url.trim_end_matches('/'),
            cfg.project_id
        );
        let body: serde_json::Value = match reqwest::Client::new().get(&url).send().await {
            Ok(resp) => resp.json().await.unwrap_or(serde_json::Value::Null),
            Err(e) => {
                crate::error!("launchpad round-info: rounds lookup failed: {e}");
                serde_json::Value::Null
            }
        };

        let items = body
            .get("items")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        // Prefer the open round; otherwise the most recent by opens_at.
        let pick = items
            .iter()
            .find(|r| {
                r.get("status")
                    .and_then(|s| s.as_str())
                    .map(|s| s.eq_ignore_ascii_case("open"))
                    .unwrap_or(false)
            })
            .or_else(|| {
                items.iter().max_by_key(|r| {
                    r.get("opens_at").and_then(|v| v.as_i64()).unwrap_or(0)
                })
            });

        let Some(r) = pick else {
            return Ok(LaunchpadRoundInfo::default());
        };

        let get_i64 = |k: &str| r.get(k).and_then(|v| v.as_i64()).unwrap_or(0);
        let status = r
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_lowercase();

        Ok(LaunchpadRoundInfo {
            has_round: true,
            name: r
                .get("name")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or("Round")
                .to_string(),
            status,
            total_points_registered: get_i64("total_points_registered"),
            total_entries: get_i64("total_entries"),
            opens_at: get_i64("opens_at"),
            closes_at: get_i64("closes_at"),
        })
    }
}
