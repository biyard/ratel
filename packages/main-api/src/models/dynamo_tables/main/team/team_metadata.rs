use super::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum TeamMetadata {
    Team(Team),
    TeamOwner(TeamOwner),
    TeamGroup(TeamGroup),
}
