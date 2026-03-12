use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::*;

use crate::common::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceAction {
    pub pk: CompositePartition<SpacePartition, String>,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(prefix = "SA", name = "find_by_space", index = "gsi1", pk)]
    pub space_pk: Partition,

    pub title: String,
    pub description: String,
    pub space_action_type: SpaceActionType,
    pub prerequisite: bool,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub started_at: i64,
    pub ended_at: i64,

    // Reward fields (migrated from ActionReward)
    pub credits: u64,
    pub boost_multiplier: u64,
    pub total_reward: u64,
}

#[cfg(feature = "server")]
impl SpaceAction {
    pub fn new(
        space_id: SpacePartition,
        action_id: String,
        space_action_type: SpaceActionType,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let space_pk: Partition = space_id.clone().into();

        Self {
            pk: CompositePartition(space_id, action_id),
            sk: EntityType::SpaceAction,
            space_pk,
            title: String::new(),
            description: String::new(),
            space_action_type,
            prerequisite: false,
            created_at: now,
            updated_at: now,
            started_at: now,
            ended_at: now + 7 * 24 * 60 * 60 * 1000, // Default 7 days
            credits: 0,
            boost_multiplier: 0,
            total_reward: 0,
        }
    }

    pub fn set_credits(mut self, credits: u64) -> Self {
        self.credits = credits;
        self.boost_multiplier = credits;
        self.total_reward = credits * 10_000;
        self
    }
}
