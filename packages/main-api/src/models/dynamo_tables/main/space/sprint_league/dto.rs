use bdk::prelude::*;

use crate::models::{SprintLeagueMetadata, SprintLeaguePlayer};

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct SprintLeagueResponse {
    pub players: Vec<SprintLeaguePlayer>,
    pub votes: i64,
    pub is_voted: bool,
}

impl From<Vec<SprintLeagueMetadata>> for SprintLeagueResponse {
    fn from(entity: Vec<SprintLeagueMetadata>) -> Self {
        let mut res = Self::default();
        for entry in entity {
            match entry {
                SprintLeagueMetadata::Space(sprint) => {
                    res.votes = sprint.voters;
                }
                SprintLeagueMetadata::Player(player) => {
                    res.players.push(player);
                }
            }
        }
        res
    }
}
