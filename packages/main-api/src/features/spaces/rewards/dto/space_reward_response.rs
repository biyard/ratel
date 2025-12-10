use bdk::prelude::*;

use crate::features::spaces::rewards::{RewardType, SpaceReward};
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
    pub sk: RewardType,

    pub created_at: i64,
    pub updated_at: i64,

    pub label: String,
    pub description: String,
    pub amount: i64,
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
            amount: reward.point,
        }
    }
}
