use crate::common::*;
use crate::features::character::leveling;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
pub struct CharacterXp {
    pub pk: Partition,  // Partition::User(user_id)
    pub sk: EntityType, // EntityType::CharacterXp

    pub created_at: i64,
    pub updated_at: i64,

    /// Monotonic — sum of every SpaceScore delta the user has accumulated.
    pub total_xp: i64,
    /// Derived from `total_xp`; denormalized so the page is a single point read.
    pub level: i32,
    /// `5 · level` — granted SP, monotonic.
    pub total_sp_granted: i32,
    /// Sum of skill costs paid via the level-up endpoint.
    pub total_sp_spent: i32,
}

impl CharacterXp {
    pub fn unspent_sp(&self) -> i32 {
        self.total_sp_granted - self.total_sp_spent
    }

    pub fn user_keys(user_pk: &Partition) -> (Partition, EntityType) {
        (user_pk.clone(), EntityType::CharacterXp)
    }
}

#[cfg(feature = "server")]
impl CharacterXp {
    pub fn new(user_pk: Partition) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: user_pk,
            sk: EntityType::CharacterXp,
            created_at: now,
            updated_at: now,
            total_xp: 0,
            level: 1,
            // 5 SP at L1 from day 0
            total_sp_granted: leveling::total_sp_granted(1),
            total_sp_spent: 0,
        }
    }
}
