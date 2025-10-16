use bdk::prelude::*;

use crate::models::SpaceCommon;

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct SprintLeagueResponse {
    #[serde(flatten)]
    pub common: SpaceCommon,

    pub players: Vec<SprintLeaguePlayer>,
    pub votes: i64,
    pub is_voted: bool,
}

impl From<Vec<SprintLeagueSpaceMetadata>> for SprintLeagueResponse {
    fn from(entity: Vec<SprintLeagueSpaceMetadata>) -> Self {
        let mut res = Self::default();
        for entry in entity {
            match entry {
                SprintLeagueSpaceMetadata::Common(common) => res.common = common,
                SprintLeagueSpaceMetadata::Space(sprint) => {
                    res.votes = sprint.voters;
                }
                SprintLeagueSpaceMetadata::Player(player) => {
                    res.players.push(player);
                }
            }
        }
        res
    }
}
