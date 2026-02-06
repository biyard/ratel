pub mod create_space_incentive;
pub use create_space_incentive::*;

pub mod get_space_incentive;
pub use get_space_incentive::*;

pub mod list_space_incentive_candidates;
pub use list_space_incentive_candidates::*;

pub mod get_space_incentive_user;
pub use get_space_incentive_user::*;

pub mod create_space_incentive_users;
pub use create_space_incentive_users::*;

pub mod list_space_incentive_tokens;
pub use list_space_incentive_tokens::*;

pub mod refresh_space_incentive_tokens;
pub use refresh_space_incentive_tokens::*;

pub mod update_space_incentive_users;
pub use update_space_incentive_users::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_space_incentive_handler).post(create_space_incentive_handler),
        )
        .route(
            "/user",
            get(get_space_incentive_user_handler)
                .post(create_space_incentive_users_handler)
                .patch(update_space_incentive_users_handler),
        )
        .route("/candidates", get(list_space_incentive_candidates_handler))
        .route("/tokens", get(list_space_incentive_tokens_handler))
        .route("/tokens/refresh", post(refresh_space_incentive_tokens_handler))
}
