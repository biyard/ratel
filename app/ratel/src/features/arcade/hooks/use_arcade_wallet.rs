//! `UseArcadeWallet` — chip balance + RP→chip exchange controller.
//!
//! Owned by `ArcadeLayout` so every arcade page shares one wallet
//! state. The chip-balance widget in the header reads `state()?` and
//! the exchange modal calls `convert(rp)` which bumps the refresh
//! signal so any consumer's `use_loader` re-fetches.
//!
//! Loader-resolution convention: methods on this struct return
//! `Result<Loader<T>, Loading>` so the suspension happens at the
//! component call site (per dev convention 2026-05-19), not at the
//! provider level — components inside Suspense boundaries can render
//! independently while the loader is still pending.

use crate::features::arcade::{
    convert_rp_handler, get_wallet_handler, ConvertRpRequest, ConvertRpResponse,
    WalletStateResponse,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseArcadeWallet {
    /// Bump to force `state()` consumers to re-fetch.
    pub state_refresh: Signal<u64>,
}

impl UseArcadeWallet {
    /// Loader handle for the wallet state. Components `?`-suspend at
    /// their own site.
    pub fn state(&self) -> std::result::Result<Loader<WalletStateResponse>, Loading> {
        let refresh = self.state_refresh;
        use_loader(move || async move {
            let _ = refresh();
            get_wallet_handler().await
        })
    }

    pub async fn convert(&mut self, rp_amount: i64) -> crate::common::Result<ConvertRpResponse> {
        let res = convert_rp_handler(ConvertRpRequest { rp_amount }).await?;
        self.state_refresh.with_mut(|n| *n += 1);
        Ok(res)
    }
}

#[track_caller]
pub fn use_arcade_wallet_provider() -> std::result::Result<UseArcadeWallet, RenderError> {
    if let Some(ctx) = try_use_context::<UseArcadeWallet>() {
        return Ok(ctx);
    }

    let state_refresh = use_signal(|| 0u64);

    Ok(use_context_provider(|| UseArcadeWallet { state_refresh }))
}

pub fn use_arcade_wallet() -> UseArcadeWallet {
    use_context::<UseArcadeWallet>()
}
