use crate::features::spaces::rewards::{RewardCondition, RewardPeriod, RewardType};
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
    pub sk: RewardType,

    pub created_at: i64,
    pub updated_at: i64,

    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl Reward {
    pub fn new(
        reward_type: RewardType,
        point: i64,
        period: RewardPeriod,
        condition: RewardCondition,
    ) -> Self {
        let now = now();

        Self {
            pk: Partition::Reward,
            sk: reward_type,
            created_at: now,
            updated_at: now,

            point,
            period,
            condition,
        }
    }

    pub async fn get_by_reward_type(
        cli: &aws_sdk_dynamodb::Client,
        reward_type: &RewardType,
    ) -> Result<Self> {
        let pk = Partition::Reward;
        let m = Reward::get(cli, pk, Some(reward_type))
            .await?
            .ok_or(Error::RewardNotFound)?;

        Ok(m)
    }
}
