use std::fmt::Display;

use chrono::Datelike;

use crate::{
    features::spaces::rewards::{FeatureRewardTrait, RewardCondition, RewardKey, RewardPeriod},
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
pub enum BoardRewardKey {
    #[default]
    Default,
    Comment(String),

    Like(String),
}

impl FeatureRewardTrait for BoardRewardKey {
    fn is_empty(&self) -> bool {
        false
    }
}

impl From<(SpacePostEntityType, BoardRewardKey)> for RewardKey {
    fn from(value: (SpacePostEntityType, BoardRewardKey)) -> Self {
        Self::Board(value.0, value.1)
    }
}
