use crate::common::utils::time::get_now_timestamp_millis;

use crate::features::spaces::pages::actions::*;

use crate::common::macros::DynamoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpaceAction {
    pub pk: CompositePartition<Partition, EntityType>,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    // Reward fields (migrated from ActionReward)
    pub credits: u64,
    pub boost_multiplier: u64,
    pub total_reward: u64,
}

#[cfg(feature = "server")]
impl SpaceAction {
    pub fn new(space_pk: Partition, action_sk: EntityType) -> Self {
        let now = get_now_timestamp_millis();

        Self {
            pk: CompositePartition(space_pk, action_sk),
            sk: EntityType::SpaceAction,
            created_at: now,
            updated_at: now,
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
