use std::{fmt::Display, str::FromStr};

use chrono::Datelike;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{
    features::spaces::rewards::{RewardAction, RewardCondition, RewardPeriod, RewardUserBehavior},
    *,
};

#[derive(
    Debug,
    Clone,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    JsonSchema,
    PartialEq,
    Eq,
    OperationIo,
)]
pub struct SpaceRewardSk(pub EntityType, pub RewardUserBehavior);

impl SpaceRewardSk {
    pub fn get_action(&self) -> RewardAction {
        match self {
            Self(_, behavior) => behavior.action(),
        }
    }
    pub fn get_sk_prefix(entity_type: EntityType) -> String {
        format!("{}##{}", EntityType::SpaceReward, entity_type)
    }
}

impl Display for SpaceRewardSk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}##{}##{}", EntityType::SpaceReward, self.0, self.1)
    }
}
impl FromStr for SpaceRewardSk {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split("##").collect();
        if parts.len() != 3 {
            return Err(Error::InvalidEntityType);
        }
        if parts[0] != EntityType::SpaceReward.to_string() {
            return Err(Error::InvalidEntityType);
        }
        let entity_type = EntityType::from_str(parts[1])?;
        let behavior = RewardUserBehavior::from_str(parts[2])?;
        Ok(SpaceRewardSk(entity_type, behavior))
    }
}
