// pub mod create_reward;
// pub use create_reward::*;

// pub mod list_rewards;
// pub use list_rewards::*;

// pub mod delete_reward;
// pub use delete_reward::*;

// pub mod update_reward;
// pub use update_reward::*;

// #[cfg(test)]
// pub mod tests;

use crate::AppState;
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new()
    // Router::new().route(
    // "/",
    // get(list_rewards_handler)
    //     .post(create_reward_handler)
    //     .put(update_reward_handler)
    //     .delete(delete_reward_handler),
    // )
    // .route(
    //     "/:reward_sk",
    //     put(update_reward_handler).delete(delete_reward_handler),
    // )
}
