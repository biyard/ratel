use crate::features::spaces::rewards::{RewardCondition, RewardKey, RewardPeriod};
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
    pub sk: RewardKey,

    pub created_at: i64,
    pub updated_at: i64,

    pub label: String,
    pub description: String,

    pub credits: i64,
    pub point: i64,

    pub total_points: i64,
    pub total_claims: i64,

    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl SpaceReward {
    pub fn new(
        space_pk: SpacePartition,
        reward_key: RewardKey,
        label: String,
        description: String,
        credits: i64,
        point: i64,
        period: RewardPeriod,
        condition: RewardCondition,
    ) -> Self {
        let (pk, sk) = Self::keys(space_pk, reward_key);
        let now = now();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,

            label,
            credits,
            point,
            description,

            period,
            condition,
            total_points: 0,
            total_claims: 0,
        }
    }

    pub fn keys(
        space_pk: SpacePartition,
        reward_key: RewardKey,
    ) -> (CompositePartition, RewardKey) {
        // SPACE#{space_pk}##REWARD
        (
            CompositePartition(space_pk.into(), Partition::Reward),
            reward_key,
        )
    }

    pub fn get_amount(&self) -> i64 {
        self.point * self.credits
    }

    pub fn get_space_pk(&self) -> SpacePartition {
        self.pk.0.clone().into()
    }

    pub async fn get_by_reward_key(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: SpacePartition,
        reward_key: RewardKey,
    ) -> Result<Self> {
        let key = Self::keys(space_pk, reward_key);
        let res = Self::get(cli, key.0, Some(key.1))
            .await?
            .ok_or(Error::RewardNotFound)?;
        Ok(res)
    }

    pub async fn list_by_feature(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: SpacePartition,
        entity_type: Option<EntityType>,
        bookmark: Option<String>,
    ) -> Result<(Vec<Self>, Option<String>)> {
        let pk: CompositePartition = CompositePartition(space_pk.into(), Partition::Reward);
        let mut opt = SpaceRewardQueryOption::builder();
        if let Some(bookmark) = bookmark {
            opt = opt.bookmark(bookmark);
        }

        if let Some(entity_type) = entity_type {
            let begin_sk = RewardKey::get_feature_begin_sk(entity_type);
            opt = opt.sk(begin_sk);
        }

        let (items, next) = Self::query(cli, pk, opt).await?;

        Ok((items, next))
    }
}
