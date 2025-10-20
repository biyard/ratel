use bdk::prelude::*;

use crate::types::{EntityType, Partition};

use super::super::{SprintLeague, SprintLeagueMetadata, SprintLeaguePlayer};

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct SprintLeagueResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub players: Vec<SprintLeaguePlayer>,

    pub votes: i64,
    pub win_player: Option<EntityType>,

    pub is_voted: bool,
}

impl From<Vec<SprintLeagueMetadata>> for SprintLeagueResponse {
    fn from(entity: Vec<SprintLeagueMetadata>) -> Self {
        let mut res = Self::default();
        for entry in entity {
            match entry {
                SprintLeagueMetadata::SprintLeague(sprint) => {
                    res.votes = sprint.votes;
                }
                SprintLeagueMetadata::SprintLeaguePlayer(player) => {
                    res.players.push(player);
                }
            }
        }
        res
    }
}

impl From<(SprintLeague, Vec<SprintLeaguePlayer>, bool)> for SprintLeagueResponse {
    fn from(
        (sprint_league, players, is_voted): (SprintLeague, Vec<SprintLeaguePlayer>, bool),
    ) -> Self {
        Self {
            pk: sprint_league.pk,
            sk: sprint_league.sk,
            players,
            votes: sprint_league.votes,
            win_player: sprint_league.win_player,
            is_voted,
        }
    }
}
