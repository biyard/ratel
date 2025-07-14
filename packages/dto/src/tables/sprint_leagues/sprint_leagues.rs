use bdk::prelude::*;

use crate::{
    SprintLeaguePlayer, SprintLeaguePlayerCreateRequest, SprintLeaguePlayerRepositoryQueryBuilder,
};

#[api_model(base = "/spaces/:space_id/sprint-leagues", table = sprint_leagues, action = [create(players = Vec<SprintLeaguePlayerCreateRequest>)], action_by_id = vote(player_id = i64, referral_code = Option<String>))]
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

    #[api_model(one_to_many = sprint_league_players, foreign_key = sprint_league_id, nested)]
    pub players: Vec<SprintLeaguePlayer>,

    #[api_model(skip)]
    #[serde(default)]
    pub winner_id: Option<i64>,

    #[api_model(action = create)]
    #[serde(default)]
    pub reward_amount: i64,
}
