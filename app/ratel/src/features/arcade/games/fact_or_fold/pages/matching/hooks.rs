//! `UseFactFoldMatching` — owns the lobby poll + leave action for
//! the pre-game waiting room.

use crate::features::arcade::games::fact_or_fold::{
    get_lobby_handler, leave_lobby_handler, LobbyResponse, RoundResponse,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldMatching {
    pub lobby: Loader<LobbyResponse>,
}

impl UseFactFoldMatching {
    pub async fn leave(&mut self) -> crate::common::Result<RoundResponse> {
        let res = leave_lobby_handler().await?;
        self.lobby.restart();
        Ok(res)
    }
}

#[track_caller]
pub fn use_fact_fold_matching_provider() -> std::result::Result<UseFactFoldMatching, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldMatching>() {
        return Ok(ctx);
    }
    let lobby = use_loader(move || async move { get_lobby_handler().await })?;
    Ok(use_context_provider(|| UseFactFoldMatching { lobby }))
}
