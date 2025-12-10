use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{
    features::spaces::rewards::{BoardReward, PollReward, RewardCondition, RewardPeriod},
    *,
};

#[derive(
    Debug,
    Clone,
    DynamoEnum,
    Default,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
    OperationIo,
    Eq,
    PartialEq,
)]
pub enum RewardKey {
    #[default]
    Default,
    Poll(SpacePollEntityType, PollReward),
    Board(SpacePostEntityType, BoardReward),
}

pub trait FeatureRewardTrait {
    fn is_empty(&self) -> bool;
}
