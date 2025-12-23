use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::features::spaces::rewards::{RewardCondition, RewardPeriod};
use crate::*;

#[derive(
    Debug,
    Clone,
    DynamoEnum,
    SerializeDisplay,
    DeserializeFromStr,
    JsonSchema,
    OperationIo,
    Eq,
    PartialEq,
    Default,
)]
pub enum RewardType {
    #[default]
    PollRespond,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct RewardConfig {
    pub reward_type: RewardType,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl RewardType {
    pub fn all() -> Vec<RewardConfig> {
        vec![RewardConfig {
            reward_type: RewardType::PollRespond,
            point: 10_000,
            period: RewardPeriod::Once,
            condition: RewardCondition::None,
        }]
    }

    pub fn config(&self) -> RewardConfig {
        match self {
            RewardType::PollRespond => RewardConfig {
                reward_type: self.clone(),
                point: 10_000,
                period: RewardPeriod::Once,
                condition: RewardCondition::None,
            },
        }
    }

    pub fn poll_types() -> Vec<RewardConfig> {
        vec![RewardType::PollRespond.config()]
    }
}
