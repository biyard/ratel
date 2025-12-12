use bdk::prelude::*;

use crate::features::spaces::rewards::{
    RewardCondition, RewardKey, RewardPeriod, SpaceReward, UserReward,
};
use crate::types::{CompositePartition, Partition};

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceRewardResponse {
    pub pk: CompositePartition,
    pub sk: RewardKey,

    pub created_at: i64,
    pub updated_at: i64,

    pub label: String,
    pub description: String,
    pub points: i64,
    pub credits: i64,

    pub total_points: i64,
    pub total_claims: i64,

    pub period: RewardPeriod,
    pub condition: RewardCondition,

    pub user_claims: i64,
    pub user_points: i64,
}
// match
// USER_CLAIMS / RewardCondition

impl From<(SpaceReward, UserReward)> for SpaceRewardResponse {
    fn from((reward, user_reward): (SpaceReward, UserReward)) -> Self {
        Self {
            pk: reward.pk,
            sk: reward.sk,
            created_at: reward.created_at,
            updated_at: reward.updated_at,
            label: reward.label,
            description: reward.description,

            points: reward.point,
            credits: reward.credits,
            total_points: reward.total_points,
            total_claims: reward.total_claims,

            period: reward.period,
            condition: reward.condition,

            user_claims: user_reward.total_claims,
            user_points: user_reward.total_points,
        }
    }
}

impl From<SpaceReward> for SpaceRewardResponse {
    fn from(reward: SpaceReward) -> Self {
        Self {
            pk: reward.pk,
            sk: reward.sk,
            created_at: reward.created_at,
            updated_at: reward.updated_at,
            label: reward.label,
            description: reward.description,

            points: reward.point,
            credits: reward.credits,
            total_points: reward.total_points,
            total_claims: reward.total_claims,

            period: reward.period,
            condition: reward.condition,

            user_claims: 0,
            user_points: 0,
        }
    }
}
