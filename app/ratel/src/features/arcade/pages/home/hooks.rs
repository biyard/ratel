//! `UseArcadeHome` — bundles every loader + mutation the home page
//! needs: lobby state, my stats, leaderboard page, and the join
//! mutation (which routes to /matching on success).

use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::features::arcade::games::fact_or_fold::{
    get_leaderboard_handler, get_lobby_handler, get_my_stats_handler, join_lobby_handler,
    LeaderboardEntryResponse, LobbyResponse, RoundResponse, UserStatsResponse,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseArcadeHome {
    pub lobby: Loader<LobbyResponse>,
    pub my_stats: Loader<UserStatsResponse>,
    pub leaderboard:
        InfiniteQuery<String, LeaderboardEntryResponse, ListResponse<LeaderboardEntryResponse>>,
}

impl UseArcadeHome {
    pub async fn join(&mut self) -> crate::common::Result<RoundResponse> {
        let res = join_lobby_handler().await?;
        self.lobby.restart();
        Ok(res)
    }
}

#[track_caller]
pub fn use_arcade_home_provider() -> std::result::Result<UseArcadeHome, RenderError> {
    if let Some(ctx) = try_use_context::<UseArcadeHome>() {
        return Ok(ctx);
    }

    let lobby = use_loader(move || async move { get_lobby_handler().await })?;
    let my_stats = use_loader(move || async move { get_my_stats_handler().await })?;
    let leaderboard = use_infinite_query(move |bookmark| async move {
        get_leaderboard_handler(bookmark).await
    })?;

    Ok(use_context_provider(|| UseArcadeHome {
        lobby,
        my_stats,
        leaderboard,
    }))
}
