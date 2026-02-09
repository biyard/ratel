use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{
    features::spaces::rewards::{Reward, RewardUserBehavior},
    types::*,
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
pub struct RewardKey {
    pub space_pk: Option<SpacePartition>,
    pub entity_type: Option<EntityType>,
    pub behavior: RewardUserBehavior,
}

impl Display for RewardKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.space_pk, &self.entity_type) {
            (Some(space), Some(entity)) => {
                write!(
                    f,
                    "{}##{}##{}##{}",
                    EntityType::SpaceReward,
                    space,
                    entity,
                    self.behavior
                )
            }
            (None, None) => {
                write!(f, "{}##{}", EntityType::Reward, self.behavior)
            }
            _ => Err(std::fmt::Error),
        }
    }
}

impl FromStr for RewardKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("##").collect();

        if parts.len() < 2 {
            return Err(Error::InvalidEntityType);
        }

        let entity_type = EntityType::from_str(parts[0]).map_err(|_| Error::InvalidEntityType)?;

        match entity_type {
            EntityType::Reward => {
                if parts.len() == 2 {
                    let behavior = RewardUserBehavior::from_str(parts[1])?;
                    Ok(RewardKey {
                        space_pk: None,
                        entity_type: None,
                        behavior,
                    })
                } else {
                    Err(Error::InvalidEntityType)
                }
            }
            EntityType::SpaceReward => {
                if parts.len() != 4 {
                    return Err(Error::InvalidEntityType);
                }

                let space_pk = SpacePartition::from_str(parts[1])?;
                let entity_type = EntityType::from_str(parts[2])?;
                let behavior = RewardUserBehavior::from_str(parts[3])?;

                Ok(RewardKey {
                    space_pk: Some(space_pk),
                    entity_type: Some(entity_type),
                    behavior,
                })
            }
            _ => Err(Error::InvalidEntityType),
        }
    }
}

impl From<(SpacePartition, EntityType, RewardUserBehavior)> for RewardKey {
    fn from(
        (space_pk, entity_type, behavior): (SpacePartition, EntityType, RewardUserBehavior),
    ) -> Self {
        Self {
            space_pk: Some(space_pk),
            entity_type: Some(entity_type),
            behavior,
        }
    }
}

impl From<Reward> for RewardKey {
    fn from(reward: Reward) -> Self {
        Self {
            space_pk: None,
            entity_type: None,
            behavior: reward.sk,
        }
    }
}

impl RewardKey {
    pub fn get_space_reward_sk_prefix(
        space_pk: SpacePartition,
        action_key: Option<EntityType>,
    ) -> String {
        match action_key {
            Some(action_key) => {
                format!("{}##{}##{}", EntityType::SpaceReward, space_pk, action_key)
            }
            None => {
                format!("{}##{}", EntityType::SpaceReward, space_pk)
            }
        }
    }
}
