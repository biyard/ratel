use std::fmt::Display;

use chrono::Datelike;

use crate::{
    features::spaces::rewards::{FeatureRewardTrait, RewardCondition, RewardPeriod},
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
pub enum BoardReward {
    #[default]
    Default,
    Comment(String),

    Like(String),
}

impl FeatureRewardTrait for BoardReward {
    fn is_empty(&self) -> bool {
        false
    }
}
