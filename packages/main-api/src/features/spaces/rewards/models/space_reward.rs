use crate::features::spaces::rewards::{RewardCondition, RewardPeriod, RewardType};
use crate::types::*;
use crate::*;

/// SpaceReward: 스페이스에 설정한 리워드
///
/// Key Structure:
/// - PK: SPACE#{space_pk}##REWARD
/// - SK: {EntityType}#{RewardType}
///
/// Examples:
/// - Get All Rewards: SpaceReward::query(pk)
/// - Get Specific Entity Reward: SpaceReward::query_begins_with_sk(EntityType)
/// - Get Specific Reward: SpaceReward::get(pk, sk)

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
/// SpaceReward: 스페이스에 설정한 리워드
///
/// Key Structure:
/// - PK: SPACE#{space_pk}##REWARD
/// - SK: {EntityType}#{RewardType}
///
/// Examples:
/// - Get All Rewards: SpaceReward::query(pk)
/// - Get Specific Entity Reward: SpaceReward::query_begins_with_sk(EntityType)
/// - Get Specific Reward: SpaceReward::get(pk, sk)
pub struct SpaceReward {
    pub pk: CompositePartition,
    pub sk: RewardType,

    pub created_at: i64,
    pub updated_at: i64,

    pub label: String,
    pub description: String,

    pub point: i64,

    pub total_points: i64,
    pub total_claims: i64,

    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl SpaceReward {
    pub fn new(
        space_pk: SpacePartition,
        reward_type: RewardType,
        label: String,
        description: String,
        credits: i64,
    ) -> Self {
        let detail = reward_type.detail();

        let (pk, sk) = Self::keys(space_pk, reward_type);
        let now = time::get_now_timestamp_millis();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,

            label,
            point: detail.point * credits,
            description,

            period: detail.period,
            condition: detail.condition,
            ..Default::default()
        }
    }

    pub fn keys(
        space_pk: SpacePartition,
        reward_type: RewardType,
    ) -> (CompositePartition, RewardType) {
        // SPACE#{space_pk}##REWARD
        (
            CompositePartition(space_pk.into(), Partition::Reward),
            reward_type,
        )
    }

    pub fn get_space_pk(&self) -> SpacePartition {
        self.pk.0.clone().into()
    }

    pub async fn get_by_reward_type(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: SpacePartition,
        reward_type: RewardType,
    ) -> Result<Self> {
        let key = Self::keys(space_pk, reward_type);
        let res = Self::get(cli, key.0, Some(key.1))
            .await?
            .ok_or(Error::RewardNotFound)?;
        Ok(res)
    }
}
