use std::fmt::Display;

use chrono::Datelike;

use crate::{
    features::spaces::rewards::{
        FeatureRewardTrait, RewardCondition, RewardKey, RewardPeriod, RewardType,
    },
    *,
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    DynamoEnum,
    JsonSchema,
    Default,
    serde::Serialize,
    serde::Deserialize,
    OperationIo,
)]
pub enum PollReward {
    #[default]
    Respond,
}

impl FeatureRewardTrait for PollReward {
    fn is_empty(&self) -> bool {
        false
    }
}

impl From<(SpacePollEntityType, PollReward)> for RewardKey {
    fn from(value: (SpacePollEntityType, PollReward)) -> Self {
        Self::Poll(value.0, value.1)
    }
}
