//! `UseArcadeHome` — bundles every loader + mutation the home page
//! needs: lobby state, my stats, leaderboard page, and the join
//! mutation (which routes to /matching on success).
//!
//! Loader-resolution convention: `lobby()` and `my_stats()` are
//! methods returning `Result<Loader<T>, Loading>` so the suspension
//! happens at the consumer site, not in the provider.

use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::features::arcade::games::fact_or_fold::{
    get_leaderboard_handler, get_lobby_handler, get_my_stats_handler, join_lobby_handler,
    LeaderboardEntryResponse, LobbyResponse, RoundResponse, UserStatsResponse,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseArcadeHome {
    pub lobby_refresh: Signal<u64>,
    pub my_stats_refresh: Signal<u64>,
    pub leaderboard:
        InfiniteQuery<String, LeaderboardEntryResponse, ListResponse<LeaderboardEntryResponse>>,
}

impl UseArcadeHome {
    pub fn lobby(&self) -> std::result::Result<Loader<LobbyResponse>, Loading> {
        let refresh = self.lobby_refresh;
        use_loader(move || {
            let _ = refresh();
            async move { get_lobby_handler().await }
        })
    }

    pub fn my_stats(&self) -> std::result::Result<Loader<UserStatsResponse>, Loading> {
        let refresh = self.my_stats_refresh;
        use_loader(move || {
            let _ = refresh();
            async move { get_my_stats_handler().await }
        })
    }

    pub async fn join(&mut self) -> crate::common::Result<RoundResponse> {
        let res = join_lobby_handler().await?;
        self.lobby_refresh.with_mut(|n| *n += 1);
        Ok(res)
    }
}

#[track_caller]
pub fn use_arcade_home_provider() -> std::result::Result<UseArcadeHome, RenderError> {
    if let Some(ctx) = try_use_context::<UseArcadeHome>() {
        return Ok(ctx);
    }

    let lobby_refresh = use_signal(|| 0u64);
    let my_stats_refresh = use_signal(|| 0u64);
    let leaderboard = use_infinite_query(move |bookmark| async move {
        get_leaderboard_handler(bookmark).await
    })?;

    Ok(use_context_provider(|| UseArcadeHome {
        lobby_refresh,
        my_stats_refresh,
        leaderboard,
    }))
}
