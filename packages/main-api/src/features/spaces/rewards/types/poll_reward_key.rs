use std::fmt::Display;

use chrono::Datelike;

use crate::{
    features::spaces::rewards::{
        FeatureRewardKeyTrait, RewardAction, RewardCondition, RewardKey, RewardPeriod,
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
pub enum PollRewardKey {
    #[default]
    Respond,
}

impl FeatureRewardKeyTrait for PollRewardKey {
    fn is_empty(&self) -> bool {
        false
    }

    fn get_action(&self) -> RewardAction {
        RewardAction::PollRespond
    }
}

impl From<(SpacePollEntityType, PollRewardKey)> for RewardKey {
    fn from(value: (SpacePollEntityType, PollRewardKey)) -> Self {
        Self::Poll(value.0, value.1)
    }
}
