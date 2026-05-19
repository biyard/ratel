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
    let user_id = UserPartition::from(user.pk.clone()).0;
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
        user_pk: UserPartition(user_id),
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

    // Enrich each entry with the user's display metadata in a single
    // BatchGetItem so the response is one RTT regardless of page size.
    let user_keys: Vec<(Partition, EntityType)> = rows
        .iter()
        .map(|e| (e.user_pk.clone(), EntityType::User))
        .collect();
    let user_rows = User::batch_get(cli, user_keys).await.map_err(|err| {
        crate::error!("get_leaderboard_handler user batch load failed: {err}");
        FactOrFoldError::StorageFailure
    })?;
    // Partition doesn't derive Hash so we key on the rendered string.
    let user_by_pk: std::collections::HashMap<String, User> = user_rows
        .into_iter()
        .map(|u| (u.pk.to_string(), u))
        .collect();

    let items: Vec<LeaderboardEntryResponse> = rows
        .into_iter()
        .map(|e| {
            let (username, display_name, profile_url) = user_by_pk
                .get(&e.user_pk.to_string())
                .map(|u| (u.username.clone(), u.display_name.clone(), u.profile_url.clone()))
                .unwrap_or_default();
            LeaderboardEntryResponse {
                user_pk: UserPartition::from(e.user_pk),
                username,
                display_name,
                profile_url,
                accuracy_bps: e.accuracy_bps,
                total_rounds: e.total_rounds,
                correct_count: e.correct_count,
                lifetime_delta_chips: e.lifetime_delta_chips,
                last_played_at: e.last_played_at,
            }
        })
        .collect();
    Ok((items, next_bookmark).into())
}
