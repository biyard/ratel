use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{
    features::spaces::rewards::{
        FeatureType, PollRewardKey, RewardAction, RewardCondition, RewardPeriod,
    },
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
    Poll(SpacePollEntityType, PollRewardKey),
    // Board(SpacePostEntityType, BoardRewardKey),
}

pub trait FeatureRewardKeyTrait {
    fn is_empty(&self) -> bool;
    fn get_action(&self) -> RewardAction;
}

impl RewardKey {
    pub fn get_action(&self) -> RewardAction {
        match self {
            Self::Poll(_, key) => key.get_action(),
            _ => RewardAction::None,
        }
    }
    pub fn get_sk_prefix(entity_type: EntityType) -> String {
        match entity_type {
            EntityType::SpacePoll(id) => format!(
                "{}#{}",
                FeatureType::Poll.get_sk_prefix(),
                SpacePollEntityType(id).to_string()
            ),
            // EntityType::SpacePost(id) => format!("BOARD#{}", SpacePostEntityType(id).to_string()),
            _ => "".to_string(),
        }
    }
}
