use std::fmt::Display;

use chrono::Datelike;

use crate::{
    features::spaces::rewards::{
        FeatureRewardTrait, RewardCondition, RewardDetail, RewardPeriod, RewardType,
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
    fn detail(&self) -> RewardDetail {
        match self {
            PollReward::Respond => RewardDetail {
                point: 10_000,
                period: RewardPeriod::Once,
                condition: RewardCondition::None,
            },
        }
    }
}
impl From<(SpacePollEntityType, PollReward)> for RewardType {
    fn from(value: (SpacePollEntityType, PollReward)) -> Self {
        RewardType::Poll(value.0, value.1)
    }
}
