use crate::models::{SprintLeagueSpace, space::SpaceCommon};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum SprintLeagueSpaceMetadata {
    Common(SpaceCommon),
    Space(SprintLeagueSpace),
    Player(SprintLeagueSpacePlayer),
}
