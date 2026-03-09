use std::{fmt::Display, str::FromStr};

use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::common::types::*;
use crate::common::*;

/// Composite key for reward entities in DynamoDB.
///
/// Supports two forms:
/// - Global reward: `Reward##{behavior}` (when `space_pk` and `entity_type` are `None`)
/// - Space-scoped reward: `SpaceReward##{space_pk}##{entity_type}##{behavior}`
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, PartialEq, Eq)]
pub struct RewardKey {
    pub space_pk: Option<SpacePartition>,
    pub entity_type: Option<EntityType>,
    pub behavior: RewardUserBehavior,
}

/// Serializes the key into its `##`-delimited string representation.
impl Display for RewardKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.space_pk, &self.entity_type) {
            // Space-scoped reward: SpaceReward##{space}##{entity}##{behavior}
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
            // Global reward: Reward##{behavior}
            (None, None) => {
                write!(f, "{}##{}", EntityType::Reward, self.behavior)
            }
            // Mixed Some/None is invalid
            _ => Err(std::fmt::Error),
        }
    }
}

/// Parses a `##`-delimited string back into a `RewardKey`.
/// Dispatches based on the leading entity type (`Reward` or `SpaceReward`).
impl FromStr for RewardKey {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("##").collect();

        if parts.len() < 2 {
            return Err(SpaceRewardError::InvalidEntityType.into());
        }

        let entity_type = EntityType::from_str(parts[0]).map_err(|_| SpaceRewardError::InvalidEntityType)?;

        match entity_type {
            EntityType::Reward => {
                if parts.len() == 2 {
                    let behavior = RewardUserBehavior::from_str(parts[1])
                        .map_err(|_| SpaceRewardError::InvalidEntityType)?;
                    Ok(RewardKey {
                        space_pk: None,
                        entity_type: None,
                        behavior,
                    })
                } else {
                    Err(SpaceRewardError::InvalidEntityType.into())
                }
            }
            EntityType::SpaceReward => {
                if parts.len() != 4 {
                    return Err(SpaceRewardError::InvalidEntityType.into());
                }

                let space_pk =
                    SpacePartition::from_str(parts[1]).map_err(|_| SpaceRewardError::InvalidEntityType)?;
                let entity_type =
                    EntityType::from_str(parts[2]).map_err(|_| SpaceRewardError::InvalidEntityType)?;
                let behavior = RewardUserBehavior::from_str(parts[3])
                    .map_err(|_| SpaceRewardError::InvalidEntityType)?;

                Ok(RewardKey {
                    space_pk: Some(space_pk),
                    entity_type: Some(entity_type),
                    behavior,
                })
            }
            _ => Err(SpaceRewardError::InvalidEntityType.into()),
        }
    }
}

/// Creates a space-scoped `RewardKey` from a `(SpacePartition, EntityType, RewardUserBehavior)` tuple.
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

/// Creates a global `RewardKey` from a `RewardUserBehavior` (no space scope).
impl From<RewardUserBehavior> for RewardKey {
    fn from(behavior: RewardUserBehavior) -> Self {
        Self {
            space_pk: None,
            entity_type: None,
            behavior,
        }
    }
}

impl RewardKey {
    /// Builds a sort key prefix for DynamoDB `begins_with` queries on space rewards.
    ///
    /// - With `action_key`: `SpaceReward##{space_pk}##{action_key}` (filter by action type)
    /// - Without: `SpaceReward##{space_pk}` (all rewards in the space)
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
