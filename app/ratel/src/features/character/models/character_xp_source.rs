use crate::common::*;

/// Per-(user, space) marker recording the last `SpaceScore.total_score`
/// applied to the user's CharacterXp. Used to compute the delta on each
/// SpaceScore MODIFY event so XP is idempotent under stream replay.
#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
pub struct CharacterXpSource {
    pub pk: Partition,  // Partition::User(user_id)
    pub sk: EntityType, // EntityType::CharacterXpSource(space_id)

    pub last_seen_score: i64,
    pub updated_at: i64,
}

impl CharacterXpSource {
    pub fn keys(user_pk: &Partition, space_id: &str) -> (Partition, EntityType) {
        (
            user_pk.clone(),
            EntityType::CharacterXpSource(space_id.to_string()),
        )
    }
}

#[cfg(feature = "server")]
impl CharacterXpSource {
    pub fn new(user_pk: Partition, space_id: String, last_seen_score: i64) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: user_pk,
            sk: EntityType::CharacterXpSource(space_id),
            last_seen_score,
            updated_at: now,
        }
    }
}
