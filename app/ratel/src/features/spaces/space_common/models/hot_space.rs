use crate::common::{types::*, *};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub enum HotSpaceHeat {
    Blazing,
    Trending,
    Rising,
}

impl Default for HotSpaceHeat {
    fn default() -> Self {
        HotSpaceHeat::Rising
    }
}

impl HotSpaceHeat {
    pub fn from_participants(participants: i64) -> Self {
        if participants >= 5_000 {
            HotSpaceHeat::Blazing
        } else if participants >= 500 {
            HotSpaceHeat::Trending
        } else {
            HotSpaceHeat::Rising
        }
    }
}

/// Constant value for the HotSpace ranking GSI partition key.
/// Every HotSpace row shares this so a single GSI query returns the global
/// ranked stream. If the partition ever grows hot enough to throttle, shard
/// by appending a hash bucket here (e.g. `ALL#0`..`ALL#15`) and fan-out reads.
pub const HOT_SPACE_RANK_PK: &str = "ALL";

/// Denormalized snapshot of a Space's Hot ranking.
///
/// - **PK**: `Partition::Space(uuid)` (same key space as `SpaceCommon`)
/// - **SK**: `EntityType::HotSpace` (one row per space)
/// - **GSI1**: `pk = HSR#ALL`, `sk = WindowedRankKey` — global ranked stream.
///
/// The fanout in `services::space_fanout` is the only writer. Read path is
/// `list_hot_spaces_handler`, which queries gsi1 directly with a bookmark.
#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct HotSpace {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    /// Always `HOT_SPACE_RANK_PK`. Constant gsi1 pk so all rows share one
    /// ranked stream sortable by `rank_key`.
    #[dynamo(prefix = "HSR", name = "find_by_rank", index = "gsi1", pk)]
    pub rank_pk: String,

    /// `WindowedRankKey` Display: `W{window:04}{score:020}` (base-62, descending
    /// by quality). Ascending GSI scan returns best-first within each window.
    #[dynamo(index = "gsi1", sk)]
    pub rank_key: WindowedRankKey,

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
