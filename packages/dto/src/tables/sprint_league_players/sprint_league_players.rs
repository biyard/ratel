use bdk::prelude::*;

#[api_model(base = "/", table = sprint_league_players)]
pub struct SprintLeaguePlayer {
    #[api_model(primary_key)]
    pub id: i64,

    #[api_model(many_to_one = sprint_leagues, action = create)]
    pub sprint_league_id: i64,

    #[api_model(action = [create, update])]
    pub name: String,

    #[api_model(action = [create, update])]
    pub description: String,

    #[api_model(action = [create, update], type= JSONB)]
    pub player_images: PlayerImages,

    #[api_model(one_to_many = sprint_league_votes, foreign_key = sprint_league_player_id, aggregator = count)]
    #[serde(default)]
    pub votes: i64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PlayerImages {
    pub select: SpriteSheet,
    pub run: SpriteSheet,
    pub win: String,
    pub lose: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpriteSheet {
    pub json: String,
    pub image: String,
}
