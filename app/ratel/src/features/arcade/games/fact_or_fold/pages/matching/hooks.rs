//! `UseFactFoldMatching` — owns the lobby poll + leave action for
//! the pre-game waiting room.

use crate::features::arcade::games::fact_or_fold::{
    get_lobby_handler, leave_lobby_handler, LobbyResponse, RoundResponse,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldMatching {
    pub lobby_refresh: Signal<u64>,
}

impl UseFactFoldMatching {
    pub fn lobby(&self) -> std::result::Result<Loader<LobbyResponse>, Loading> {
        let refresh = self.lobby_refresh;
        use_loader(move || async move {
            let _ = refresh();
            get_lobby_handler().await
        })
    }

    pub async fn leave(&mut self) -> crate::common::Result<RoundResponse> {
        let res = leave_lobby_handler().await?;
        self.lobby_refresh.with_mut(|n| *n += 1);
        Ok(res)
    }
}

#[track_caller]
pub fn use_fact_fold_matching_provider() -> std::result::Result<UseFactFoldMatching, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldMatching>() {
        return Ok(ctx);
    }
    let lobby_refresh = use_signal(|| 0u64);
    Ok(use_context_provider(|| UseFactFoldMatching {
        lobby_refresh,
    }))
}
