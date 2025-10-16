use crate::models::{SprintLeague, SprintLeaguePlayer};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum SprintLeagueMetadata {
    Space(SprintLeague),
    Player(SprintLeaguePlayer),
}
