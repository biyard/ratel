use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::gamification::*;

/// Append-only ledger entry for every XP award event.
///
/// One entry is written each time a user completes a quest (or the
/// creator receives their 10% share). The ledger is the authoritative
/// record used to backfill aggregates, replay history, or audit XP math.
///
/// DynamoDB layout: `pk = SPACE#{space_id}`,
/// `sk = SPACE_XP_LEDGER#{user_pk}#{timestamp}`. Storing per-space
/// keeps the ledger queryable with `find_by_pk` for any single space.
/// A `user_pk` GSI (`find_by_user`) lets the global profile query every
/// ledger entry for a single user across spaces.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceXpLedgerEntry {
    pub pk: Partition,  // Partition::Space
    pub sk: EntityType, // EntityType::SpaceXpLedger({user_pk}#{timestamp})

    #[dynamo(prefix = "SXL_USER", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    /// The action that generated this XP event (None for aggregate or
    /// creator-fee entries that are not tied to a single action).
    #[serde(default)]
    pub action_id: Option<String>,

    pub base_points: i64,
    pub participants_snapshot: u32,
    pub combo_multiplier: f32,
    pub streak_multiplier: f32,
    pub xp_earned: i64,

    /// True if this ledger entry is the 10% creator share rather than
    /// the user-facing XP award.
    #[serde(default)]
    pub is_creator_share: bool,
}

#[cfg(feature = "server")]
impl SpaceXpLedgerEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        space_id: SpacePartition,
        user_pk: Partition,
        action_id: Option<String>,
        base_points: i64,
        participants_snapshot: u32,
        combo_multiplier: f32,
        streak_multiplier: f32,
        xp_earned: i64,
        is_creator_share: bool,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_id.into();
        let sk = EntityType::SpaceXpLedger(format!("{user_pk}#{now}"));

        Self {
            pk,
            sk,
            user_pk,
            created_at: now,
            action_id,
            base_points,
            participants_snapshot,
            combo_multiplier,
            streak_multiplier,
            xp_earned,
            is_creator_share,
        }
    }
}
