pub mod create_reward;
pub use create_reward::*;

pub mod list_space_rewards;
pub use list_space_rewards::*;

pub mod delete_reward;
pub use delete_reward::*;

pub mod update_reward;
pub use update_reward::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new().route(
        "/",
        get(list_space_rewards_handler)
            .post(create_space_reward_handler)
            .put(update_space_reward_handler)
            .delete(delete_space_reward_handler),
    )
}
