//! Stats + leaderboard endpoints (PR7).
//!
//! Surface:
//!   GET /api/fact-or-fold/me/stats
//!   GET /api/fact-or-fold/leaderboard?bookmark
//!
//! `me/stats` returns the caller's `FactFoldUserStats` row.
//! `leaderboard` returns top-accuracy entries from the anchor pk
//! `Partition::FactFoldLeaderboard` (sk DESC, paginated via the
//! standard `ListResponse<T>` bookmark).

use crate::common::*;
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::{
    FactFoldLeaderboardEntry, FactFoldUserStats,
};

const LEADERBOARD_PAGE_LIMIT: i32 = 50;

// ── GET /api/fact-or-fold/me/stats ──────────────────────────────────

#[get("/api/fact-or-fold/me/stats", user: User)]
pub async fn get_my_stats_handler() -> Result<UserStatsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let user_id = user
        .pk
        .to_string()
        .strip_prefix("USER#")
        .unwrap_or(&user.pk.to_string())
        .to_string();
    let stats = FactFoldUserStats::get_or_default(cli, &user_id)
        .await
        .map_err(|e| {
            crate::error!("get_my_stats_handler read failed: {e}");
            FactOrFoldError::StorageFailure
        })?;
    let accuracy_bps = if stats.total_rounds > 0 {
        ((stats.correct_count * 10_000) / stats.total_rounds).clamp(0, 10_000) as i32
    } else {
        0
    };
    Ok(UserStatsResponse {
        user_pk: user.pk.to_string(),
        total_rounds: stats.total_rounds,
        correct_count: stats.correct_count,
        accuracy_bps,
        lifetime_delta_chips: stats.lifetime_delta_chips,
        last_played_at: stats.last_played_at,
    })
}

// ── GET /api/fact-or-fold/leaderboard?bookmark ──────────────────────

#[get("/api/fact-or-fold/leaderboard?bookmark", _user: User)]
pub async fn get_leaderboard_handler(
    bookmark: Option<String>,
) -> Result<ListResponse<LeaderboardEntryResponse>> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // The DynamoEntity macro's `query()` uses
    // `scan_index_forward(false)`, i.e. DESC by sk. Our sk
    // encoding is `{accuracy_bps:010}#{user_id}` so a DESC scan
    // returns the highest-accuracy users first.
    let opts = FactFoldLeaderboardEntry::opt_with_bookmark(bookmark)
        .sk("FACT_FOLD_LEADERBOARD_ENTRY".to_string())
        .limit(LEADERBOARD_PAGE_LIMIT);
    let (rows, next_bookmark) =
        FactFoldLeaderboardEntry::query(cli, FactFoldLeaderboardEntry::anchor_pk(), opts)
            .await
            .map_err(|e| {
                crate::error!("get_leaderboard_handler query failed: {e}");
                FactOrFoldError::StorageFailure
            })?;

    let items: Vec<LeaderboardEntryResponse> = rows
        .into_iter()
        .map(|e| LeaderboardEntryResponse {
            user_pk: e.user_pk.to_string(),
            accuracy_bps: e.accuracy_bps,
            total_rounds: e.total_rounds,
            correct_count: e.correct_count,
            lifetime_delta_chips: e.lifetime_delta_chips,
            last_played_at: e.last_played_at,
        })
        .collect();
    Ok((items, next_bookmark).into())
}
