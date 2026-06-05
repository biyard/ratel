//! Token holdings for the signed-in user (scope-A / D3).
//!
//! Calls launchpad's EXISTING signed external API
//! `POST /api/external/v1/token/balance` with the ratel user id as
//! `company_user_id`. launchpad resolves the linked launchpad user's
//! wallet and reads the on-chain balance. No launchpad changes, no local
//! ethers — ratel just signs (HMAC) and queries.

use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct LaunchpadTokenBalance {
    /// Token symbol (e.g. "COM"); empty when unavailable.
    pub symbol: String,
    /// Human-readable balance (scaled by token decimals).
    pub balance: String,
    /// False when the company user isn't linked / has no wallet yet.
    pub has_wallet: bool,
}

#[get("/api/launchpad/token-balance", user: crate::features::auth::User)]
pub async fn launchpad_token_balance_handler() -> crate::common::Result<LaunchpadTokenBalance> {
    server_impl::fetch(user.id()).await
}

#[cfg(feature = "server")]
mod server_impl {
    use super::LaunchpadTokenBalance;
    use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
    use crate::features::launchpad_partner::crypto::sign_callback;

    fn human(raw: &str, decimals: u8) -> String {
        let Ok(v) = raw.trim().parse::<u128>() else {
            return "0".to_string();
        };
        let div = 10u128.pow(decimals as u32);
        if div == 0 {
            return v.to_string();
        }
        let whole = v / div;
        let frac = v % div;
        if frac == 0 {
            return whole.to_string();
        }
        let frac_str = format!("{:0width$}", frac, width = decimals as usize);
        let frac_trim = frac_str.trim_end_matches('0');
        format!("{}.{}", whole, frac_trim)
    }

    pub async fn fetch(company_user_id: String) -> crate::common::Result<LaunchpadTokenBalance> {
        let cfg = LaunchpadPartnerConfig::default();

        let body = serde_json::json!({
            "project_id": cfg.project_id.clone(),
            "company_user_id": company_user_id,
        });
        let body_text = serde_json::to_string(&body).unwrap_or_default();
        let timestamp = crate::common::utils::time::get_now_timestamp_millis().to_string();
        let Some(signature) = sign_callback(&cfg.shared_secret, &timestamp, &body_text) else {
            return Ok(LaunchpadTokenBalance::default());
        };

        let url = format!(
            "{}/api/external/v1/token/balance",
            cfg.base_url.trim_end_matches('/')
        );
        let resp: serde_json::Value = match reqwest::Client::new()
            .post(&url)
            .header("content-type", "application/json")
            .header("x-launchpad-project-id", cfg.project_id.clone())
            .header("x-launchpad-timestamp", &timestamp)
            .header("x-launchpad-signature", signature)
            .body(body_text)
            .send()
            .await
        {
            Ok(r) => r.json().await.unwrap_or(serde_json::Value::Null),
            Err(e) => {
                crate::error!("launchpad token-balance: request failed: {e}");
                serde_json::Value::Null
            }
        };

        let symbol = resp
            .get("symbol")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let decimals = resp.get("decimals").and_then(|v| v.as_u64()).unwrap_or(18) as u8;
        let balance_raw = resp
            .get("balance_raw")
            .and_then(|v| v.as_str())
            .unwrap_or("0");
        let wallet = resp
            .get("wallet_address")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        Ok(LaunchpadTokenBalance {
            symbol,
            balance: human(balance_raw, decimals),
            has_wallet: !wallet.trim().is_empty(),
        })
    }
}
