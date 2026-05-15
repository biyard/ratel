//! `UseArcadeWallet` — chip balance + RP→chip exchange controller.
//!
//! Owned by `ArcadeLayout` so every arcade page shares one wallet
//! state. The chip-balance widget in the header reads `state()` and
//! the exchange modal calls `convert(rp)` which refreshes the
//! loader on success.

use crate::features::arcade::{
    convert_rp_handler, get_wallet_handler, ConvertRpRequest, ConvertRpResponse,
    WalletStateResponse,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseArcadeWallet {
    pub state: Loader<WalletStateResponse>,
}

impl UseArcadeWallet {
    pub async fn convert(&mut self, rp_amount: i64) -> crate::common::Result<ConvertRpResponse> {
        let res = convert_rp_handler(ConvertRpRequest { rp_amount }).await?;
        self.state.restart();
        Ok(res)
    }
}

#[track_caller]
pub fn use_arcade_wallet_provider() -> std::result::Result<UseArcadeWallet, RenderError> {
    if let Some(ctx) = try_use_context::<UseArcadeWallet>() {
        return Ok(ctx);
    }

    let state = use_loader(move || async move { get_wallet_handler().await })?;

    Ok(use_context_provider(|| UseArcadeWallet { state }))
}

pub fn use_arcade_wallet() -> UseArcadeWallet {
    use_context::<UseArcadeWallet>()
}
