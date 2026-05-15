//! `UseFactFoldLobby` — player-side lobby controller.
//!
//! Loads the lobby state (current round + capacity + balance gate)
//! and exposes async fn methods for join / leave. The page restarts
//! the loader after each mutation so the join button toggles to
//! Leave (and vice versa) without an extra fetch.

use crate::features::arcade::games::fact_or_fold::{
    LobbyResponse, RoundResponse, get_lobby_handler, join_lobby_handler, leave_lobby_handler,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldLobby {
    pub state: Loader<LobbyResponse>,
}

impl UseFactFoldLobby {
    pub async fn join(&mut self) -> crate::common::Result<RoundResponse> {
        let res = join_lobby_handler().await?;
        self.state.restart();
        Ok(res)
    }

    pub async fn leave(&mut self) -> crate::common::Result<RoundResponse> {
        let res = leave_lobby_handler().await?;
        self.state.restart();
        Ok(res)
    }
}

pub fn use_fact_fold_lobby_provider() -> std::result::Result<UseFactFoldLobby, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldLobby>() {
        return Ok(ctx);
    }

    let state = use_loader(move || async move { get_lobby_handler().await })?;

    Ok(use_context_provider(|| UseFactFoldLobby { state }))
}
