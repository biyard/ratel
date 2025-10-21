mod create_sprint_league;
pub use create_sprint_league::*;

mod upsert_sprint_league;
pub use upsert_sprint_league::*;

mod get_sprint_league;
pub use get_sprint_league::*;

mod vote_sprint_league;
pub use vote_sprint_league::*;

// mod create_sprint_league_player;
// pub use create_sprint_league_player::*;

mod update_sprint_league_player;
pub use update_sprint_league_player::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_sprint_league_handler)
                .post(create_sprint_league_handler)
                .put(upsert_sprint_league_handler),
        )
        .route("/votes", post(vote_sprint_league_handler))
        // .route("/players", post(create_sprint_league_player_handler))
        .route(
            "/players/:player_sk",
            put(update_sprint_league_player_handler),
        )
}
