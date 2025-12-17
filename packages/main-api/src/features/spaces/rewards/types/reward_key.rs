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

impl RewardKey {
    pub fn get_feature_begin_sk(entity_type: EntityType) -> String {
        match entity_type {
            EntityType::SpacePoll(id) => format!("POLL#{}", SpacePollEntityType(id).to_string()),
            EntityType::SpacePost(id) => format!("BOARD#{}", SpacePostEntityType(id).to_string()),
            _ => "".to_string(),
        }
    }
}
