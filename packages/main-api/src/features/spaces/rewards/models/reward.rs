use crate::features::spaces::rewards::{
    RewardAction, RewardCondition, RewardPeriod, RewardUserBehavior,
};
use crate::types::*;
use crate::*;

#[derive(
    Debug,
    Default,
    Clone,
    DynamoEntity,
    JsonSchema,
    OperationIo,
    serde::Serialize,
    serde::Deserialize,
)]

pub struct Reward {
    pub pk: Partition,
    pub sk: RewardUserBehavior,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    #[dynamo(name = "find_by_action", prefix = "ACTION", index = "gsi1", pk)]
    pub action: RewardAction,

    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl Reward {
    pub fn new(
        user_behavior: RewardUserBehavior,
        point: i64,
        period: RewardPeriod,
        condition: RewardCondition,
    ) -> Self {
        let now = now();

        Self {
            pk: Partition::Reward,
            action: user_behavior.action(),
            sk: user_behavior,
            created_at: now,
            updated_at: now,

            point,
            period,
            condition,
        }
    }
}
