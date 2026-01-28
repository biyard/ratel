pub mod create_space_dao;
pub use create_space_dao::*;

pub mod get_space_dao;
pub use get_space_dao::*;

#[cfg(test)]
pub mod tests;

use crate::AppState;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub fn route() -> Router<AppState> {
    Router::new().route("/", get(get_space_dao_handler).post(create_space_dao_handler))
}
