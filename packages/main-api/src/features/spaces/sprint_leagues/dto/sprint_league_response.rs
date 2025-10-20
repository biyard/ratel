use bdk::prelude::*;

use crate::types::{EntityType, Partition};

use super::super::{SprintLeague, SprintLeagueMetadata, SprintLeaguePlayer};

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct SprintLeagueResponse {
    pub pk: Partition,
    pub sk: EntityType,

    pub players: Vec<SprintLeaguePlayer>,

    pub votes: i64,
    pub winner: Option<SprintLeaguePlayer>,

    pub is_voted: bool,
}

impl From<(Vec<SprintLeagueMetadata>, bool)> for SprintLeagueResponse {
    fn from((entity, is_voted): (Vec<SprintLeagueMetadata>, bool)) -> Self {
        let mut res = Self::default();
        for entry in entity {
            match entry {
                SprintLeagueMetadata::SprintLeague(sprint) => {
                    res.votes = sprint.votes;
                    res.winner = sprint.winner;
                }
                SprintLeagueMetadata::SprintLeaguePlayer(player) => {
                    res.players.push(player);
                }
            }
        }

        res.is_voted = is_voted;
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
            winner: sprint_league.winner,
            is_voted,
        }
    }
}
