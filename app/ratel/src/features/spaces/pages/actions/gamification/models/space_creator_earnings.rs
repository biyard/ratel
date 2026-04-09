use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::gamification::*;

/// Per-space singleton aggregating the 10% creator fee paid out from
/// every XP award in the space.
///
/// DynamoDB layout: `pk = SPACE#{space_id}`, `sk = SPACE_CREATOR_EARNINGS`.
/// One row per space. The `recipient` is frozen at space creation time
/// (polymorphic User or Team). For `User` recipients, the same creator
/// share also flows into their `UserGlobalXp` aggregate so they level
/// up from their own spaces; for `Team` recipients, earnings accrue on
/// the team entity only — team-internal distribution is out of V1 scope.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceCreatorEarnings {
    pub pk: Partition,  // Partition::Space
    pub sk: EntityType, // EntityType::SpaceCreatorEarnings

    pub created_at: i64,
    pub updated_at: i64,

    pub recipient: CreatorRecipient,

    pub total_xp: i64,
    pub total_points: i64,
}

#[cfg(feature = "server")]
impl SpaceCreatorEarnings {
    pub fn new(space_id: SpacePartition, recipient: CreatorRecipient) -> Self {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_id.into();

        Self {
            pk,
            sk: EntityType::SpaceCreatorEarnings,
            created_at: now,
            updated_at: now,
            recipient,
            total_xp: 0,
            total_points: 0,
        }
    }

    pub fn keys(space_pk: &Partition) -> (Partition, EntityType) {
        (space_pk.clone(), EntityType::SpaceCreatorEarnings)
    }
}
