use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::features::spaces::rewards::{FeatureType, RewardCondition, RewardPeriod};
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
pub enum RewardAction {
    #[default]
    None,
    PollRespond,
}

impl RewardAction {
    pub fn feature(&self) -> FeatureType {
        match self {
            Self::None => FeatureType::None,
            Self::PollRespond => FeatureType::Poll,
        }
    }

    pub fn all(feature: FeatureType) -> Vec<Self> {
        match feature {
            FeatureType::None => vec![],
            FeatureType::Poll => vec![Self::PollRespond],
        }
    }
}
