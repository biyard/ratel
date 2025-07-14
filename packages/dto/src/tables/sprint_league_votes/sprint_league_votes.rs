use bdk::prelude::*;

#[api_model(base = "/", table = sprint_leagues_votes)]
pub struct SprintLeagueVote {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update], version = v0.1)]
    pub updated_at: i64,

    #[api_model(action = create, nullable, version = v0.1)]
    pub referral_code: Option<String>,

    #[api_model(many_to_one = users, action = create)]
    #[serde(default)]
    pub user_id: i64,

    #[api_model(many_to_one = sprint_leagues, action = create, version = v0.2)]
    #[serde(default)]
    pub sprint_league_id: i64,

    #[api_model(many_to_one = sprint_league_players, action = create, version = v0.2)]
    #[serde(default)]
    pub sprint_league_player_id: i64,
}
