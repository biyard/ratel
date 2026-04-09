use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::gamification::*;

/// Per-user singleton tracking the global daily participation streak.
///
/// DynamoDB layout: `pk = USER#{user_id}`, `sk = USER_STREAK`.
/// One row per user. `last_active_date` is stored as `YYYY-MM-DD` in
/// the user's timezone so that midnight rollover matches the local day.
///
/// Streak multipliers (applied during XP awards):
/// - 0–2 days:  ×1.0
/// - 3–6 days:  ×1.05
/// - 7–29 days: ×1.15
/// - 30+ days:  ×1.5
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserStreak {
    pub pk: Partition,  // Partition::User
    pub sk: EntityType, // EntityType::UserStreak

    pub created_at: i64,
    pub updated_at: i64,

    pub current_streak: u32,
    pub longest_streak: u32,

    /// Last day on which the user submitted any quest. `YYYY-MM-DD` in
    /// the user's local timezone.
    pub last_active_date: String,
}

#[cfg(feature = "server")]
impl UserStreak {
    pub fn new(user_id: UserPartition) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = user_id.into();

        Self {
            pk,
            sk: EntityType::UserStreak,
            created_at: now,
            updated_at: now,
            current_streak: 0,
            longest_streak: 0,
            last_active_date: String::new(),
        }
    }

    pub fn keys(user_pk: &Partition) -> (Partition, EntityType) {
        (user_pk.clone(), EntityType::UserStreak)
    }

    /// Convert the current streak into the XP multiplier applied at award
    /// time. See the struct-level doc for the bands.
    pub fn streak_multiplier(current_streak: u32) -> f32 {
        match current_streak {
            0..=2 => 1.0,
            3..=6 => 1.05,
            7..=29 => 1.15,
            _ => 1.5,
        }
    }
}
