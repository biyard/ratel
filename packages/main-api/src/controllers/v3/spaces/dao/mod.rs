pub mod create_space_dao;
pub use create_space_dao::*;

pub mod get_space_dao;
pub use get_space_dao::*;

pub mod list_space_dao_candidates;
pub use list_space_dao_candidates::*;

pub mod get_space_dao_incentive;
pub use get_space_dao_incentive::*;

pub mod create_space_dao_incentive;
pub use create_space_dao_incentive::*;

pub mod list_space_dao_tokens;
pub use list_space_dao_tokens::*;

pub mod refresh_space_dao_tokens;
pub use refresh_space_dao_tokens::*;

pub mod update_space_dao_incentive;
pub use update_space_dao_incentive::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_space_dao_handler).post(create_space_dao_handler),
        )
        .route(
            "/incentive",
            get(get_space_dao_incentive_handler)
                .post(create_space_dao_incentive_handler)
                .patch(update_space_dao_incentive_handler),
        )
        .route("/candidates", get(list_space_dao_candidates_handler))
        .route("/tokens", get(list_space_dao_tokens_handler))
        .route("/tokens/refresh", post(refresh_space_dao_tokens_handler))
}
