use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::gamification::*;

/// Per-user singleton that aggregates XP, points, and level across every
/// space the user has participated in.
///
/// DynamoDB layout: `pk = USER#{user_id}`, `sk = USER_GLOBAL_XP`.
/// There is exactly one of these per user; `UserGlobalXp::upsert` on the
/// main table replaces or creates the singleton in one call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserGlobalXp {
    pub pk: Partition,  // Partition::User
    pub sk: EntityType, // EntityType::UserGlobalXp

    pub created_at: i64,
    pub updated_at: i64,

    pub total_xp: i64,
    pub total_points: i64,

    /// `level = floor(sqrt(total_xp / 1000)) + 1`
    pub level: u32,

    pub spaces_entered: u32,
    pub spaces_cleared: u32,
    pub quests_cleared: u32,
}

#[cfg(feature = "server")]
impl UserGlobalXp {
    pub fn new(user_id: UserPartition) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = user_id.into();

        Self {
            pk,
            sk: EntityType::UserGlobalXp,
            created_at: now,
            updated_at: now,
            total_xp: 0,
            total_points: 0,
            level: 1,
            spaces_entered: 0,
            spaces_cleared: 0,
            quests_cleared: 0,
        }
    }

    pub fn keys(user_pk: &Partition) -> (Partition, EntityType) {
        (user_pk.clone(), EntityType::UserGlobalXp)
    }

    /// Derive level from total XP using `floor(sqrt(total_xp / 1000)) + 1`.
    ///
    /// - Level 1:      0 XP
    /// - Level 5:  ~16,000 XP
    /// - Level 10: ~81,000 XP
    /// - Level 20: ~361,000 XP
    pub fn level_from_xp(total_xp: i64) -> u32 {
        let xp = total_xp.max(0) as f64;
        ((xp / 1000.0).sqrt().floor() as u32) + 1
    }
}
