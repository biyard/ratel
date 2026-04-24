use crate::common::*;
#[cfg(feature = "server")]
use crate::features::essence::models::UserEssenceStats;
use crate::features::essence::types::*;
use crate::features::auth::User;

/// Aggregate counts for the current user — backs the hero card so it can
/// show accurate totals in one roundtrip instead of paginating the index.
#[get("/api/essences/stats", user: User)]
pub async fn get_essence_stats_handler() -> Result<EssenceStatsResponse> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let stats = UserEssenceStats::get_or_default(cli, user.pk.clone()).await?;
    Ok(EssenceStatsResponse {
        total_sources: stats.total_sources,
        total_words: stats.total_words,
        total_notion: stats.total_notion,
        total_post: stats.total_post,
        total_comment: stats.total_comment,
        total_poll: stats.total_poll,
        total_quiz: stats.total_quiz,
    })
}
