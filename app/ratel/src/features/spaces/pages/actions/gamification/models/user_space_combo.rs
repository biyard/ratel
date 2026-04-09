use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::gamification::*;

/// Per-space, per-user combo counter.
///
/// DynamoDB layout: `pk = SPACE#{space_id}`,
/// `sk = USER_SPACE_COMBO#{user_pk}`. Each pair (space, user) has one
/// row that tracks how many consecutive quests the user has cleared
/// inside that space, the current multiplier, and when the last
/// completion happened. The combo breaks after 24h of inactivity.
///
/// Combo multipliers (applied during XP awards):
/// - 0–2 clears: ×1.0
/// - 3–4 clears: ×2.0
/// - 5+  clears: ×3.0
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserSpaceCombo {
    pub pk: Partition,  // Partition::Space
    pub sk: EntityType, // EntityType::UserSpaceCombo(user_pk)

    pub created_at: i64,
    pub updated_at: i64,

    pub current_streak_in_space: u32,
    pub combo_multiplier: f32,
    pub last_completion_at: i64,
}

#[cfg(feature = "server")]
impl UserSpaceCombo {
    pub fn new(space_id: SpacePartition, user_pk: &Partition) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_id.into();
        let sk = EntityType::UserSpaceCombo(user_pk.to_string());

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            current_streak_in_space: 0,
            combo_multiplier: 1.0,
            last_completion_at: 0,
        }
    }

    pub fn keys(space_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            space_pk.clone(),
            EntityType::UserSpaceCombo(user_pk.to_string()),
        )
    }

    /// Convert the in-space streak count into the XP combo multiplier.
    pub fn combo_multiplier(current_streak_in_space: u32) -> f32 {
        match current_streak_in_space {
            0..=2 => 1.0,
            3..=4 => 2.0,
            _ => 3.0,
        }
    }
}
