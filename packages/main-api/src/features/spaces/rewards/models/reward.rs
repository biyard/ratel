use crate::features::spaces::rewards::{FeatureType, RewardAction, RewardCondition, RewardPeriod};
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
    pub sk: RewardAction,

    pub created_at: i64,
    pub updated_at: i64,

    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl Reward {
    pub fn new(
        reward_action: RewardAction,
        point: i64,
        period: RewardPeriod,
        condition: RewardCondition,
    ) -> Self {
        let now = now();

        Self {
            pk: Partition::Reward,
            sk: reward_action,
            created_at: now,
            updated_at: now,

            point,
            period,
            condition,
        }
    }

    pub async fn get_by_reward_action(
        cli: &aws_sdk_dynamodb::Client,
        reward_action: &RewardAction,
    ) -> Result<Self> {
        let pk = Partition::Reward;
        let m = Reward::get(cli, pk, Some(reward_action))
            .await?
            .ok_or(Error::RewardNotFound)?;

        Ok(m)
    }

    pub async fn list_by_feature(
        cli: &aws_sdk_dynamodb::Client,
        feature: &FeatureType,
        bookmark: Option<String>,
    ) -> Result<(Vec<Self>, Option<String>)> {
        let pk = Partition::Reward;
        let begin_sk = feature.get_sk_prefix();

        let mut opt = RewardQueryOption::builder().limit(100).sk(begin_sk);
        if let Some(bookmark) = bookmark {
            opt = opt.bookmark(bookmark);
        }
        let (items, next) = Self::query(cli, pk, opt).await?;

        Ok((items, next))
    }
}
