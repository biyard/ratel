use crate::common::{types::*, *};

use super::HotSpaceHeat;

/// GSI1 partition prefix for the per-viewer hot-space stream.
/// Each viewer has their own ranked list keyed by `UHS#{viewer_id}` so a
/// single GSI query returns just the spaces fanned out to that user.
pub const USER_HOT_SPACE_RANK_PREFIX: &str = "UHS";

/// Build the per-viewer GSI1 partition key for a given viewer id.
pub fn user_hot_space_rank_pk(viewer_id: &str) -> String {
    viewer_id.to_string()
}

/// Per-viewer denormalized hot-space row.
///
/// - **PK**: `Partition::User(viewer_id)` — the user who can see the space
/// - **SK**: `EntityType::UserHotSpace(space_id)` — one row per (viewer, space) pair
/// - **GSI1**: `pk = UHS#{viewer_id}`, `sk = WindowedRankKey` — viewer-scoped ranked stream
///
/// Written by `space_fanout::upsert_hot_space` for every viewer reachable
/// from the space's author (followers + team members + author). Anonymous
/// reads continue to use the global `HotSpace` rows; logged-in reads scan
/// this table by viewer.
#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct UserHotSpace {
    pub pk: Partition,  // Partition::User(viewer_id)
    pub sk: EntityType, // EntityType::UserHotSpace(space_id)

    pub created_at: i64,
    pub updated_at: i64,

    /// `UHS#{viewer_id}` — gsi1 pk so each viewer owns their own ranked stream.
    #[dynamo(prefix = "UHS", name = "find_by_viewer", index = "gsi1", pk)]
    pub rank_pk: String,

    /// `WindowedRankKey` — same encoding as the global HotSpace row. Ascending
    /// scan returns best-first within each freshness window.
    #[dynamo(index = "gsi1", sk)]
    pub rank_key: WindowedRankKey,

    pub space_pk: Partition,
    pub post_pk: Partition,
    pub title: String,
    pub description: String,
    pub logo: String,
    pub author_display_name: String,
    pub participants: i64,
    pub rewards: i64,
    pub poll_count: i64,
    pub discussion_count: i64,
    pub quiz_count: i64,
    pub follow_count: i64,
    pub total_actions: i64,
    pub heat: HotSpaceHeat,
    pub space_created_at: i64,
}
