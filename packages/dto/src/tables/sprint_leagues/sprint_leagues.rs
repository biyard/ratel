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
    pub space_id: i64,

    #[api_model(one_to_many = sprint_league_players, foreign_key = sprint_league_id, nested)]
    #[serde(default)]
    pub players: Vec<SprintLeaguePlayer>,

    #[api_model(skip)]
    #[serde(default)]
    pub winner_id: Option<i64>,

    #[api_model(one_to_many = sprint_league_votes, foreign_key = sprint_league_id, aggregator = count)]
    #[serde(default)]
    pub votes: i64,

    #[api_model(summary, many_to_many = sprint_league_votes, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = sprint_league_id, aggregator = exist)]
    #[serde(default)]
    pub is_voted: bool,

    #[api_model(action = create)]
    pub reward_amount: i64,
}
