use bdk::prelude::*;

use crate::SprintLeaguePlayer;

#[api_model(base = "/", table = sprint_leagues)]
pub struct SprintLeague {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update], version = v0.1)]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    #[serde(default)]
    pub space_id: i64,

    #[api_model(action = create)]
    pub started_at: i64,

    #[api_model(action = create)]
    pub ended_at: i64,

    #[api_model(one_to_many = sprint_league_players, foreign_key = sprint_league_id)]
    pub players: Vec<SprintLeaguePlayer>,

    #[api_model(skip)]
    pub winner_id: Option<i64>,
}
