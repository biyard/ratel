use super::super::*;

/// Global profile response aggregating the user's XP, level, streak, and
/// high-level stats across all spaces. V1 returns only data from the
/// `UserGlobalXp` and `UserStreak` singletons; fields like
/// `creator_earnings_xp` are placeholders for future enrichment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GlobalProfileResponse {
    pub total_xp: i64,
    pub total_points: i64,
    pub level: u32,
    pub current_streak: u32,
    pub longest_streak: u32,
    /// Number of spaces (dungeons) the user has entered. V1: from UserGlobalXp.
    pub dungeons_entered: u32,
    /// Number of quests the user has cleared globally. V1: from UserGlobalXp.
    pub quests_cleared: u32,
    /// Cumulative XP earned as a space creator. V1: 0 placeholder.
    pub creator_earnings_xp: i64,
}

/// Returns the authenticated user's global gamification profile.
///
/// Reads `UserGlobalXp` and `UserStreak` singletons from DynamoDB.
/// Missing singletons are treated as zero-state (new user).
#[mcp_tool(name = "get_my_global_profile", description = "Get the authenticated user's global gamification profile including XP, level, streak, dungeons entered, and quests cleared.")]
#[get("/api/me/profile", user: crate::features::auth::User)]
pub async fn get_my_global_profile() -> Result<GlobalProfileResponse> {
    let config = crate::common::CommonConfig::default();
    let cli = config.dynamodb();

    let global_xp = UserGlobalXp::get(cli, &user.pk, Some(EntityType::UserGlobalXp))
        .await
        .ok()
        .flatten();

    let streak = UserStreak::get(cli, &user.pk, Some(EntityType::UserStreak))
        .await
        .ok()
        .flatten();

    let (total_xp, total_points, level, dungeons_entered, quests_cleared) = match global_xp {
        Some(g) => (
            g.total_xp,
            g.total_points,
            g.level,
            g.spaces_entered,
            g.quests_cleared,
        ),
        None => (0, 0, 1, 0, 0),
    };

    let (current_streak, longest_streak) = match streak {
        Some(s) => (s.current_streak, s.longest_streak),
        None => (0, 0),
    };

    Ok(GlobalProfileResponse {
        total_xp,
        total_points,
        level,
        current_streak,
        longest_streak,
        dungeons_entered,
        quests_cleared,
        creator_earnings_xp: 0, // V1 placeholder
    })
}
