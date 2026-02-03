mod teams;
pub use teams::migrate_team_handler;

use bdk::prelude::{axum::routing::*, *};

pub fn route() -> crate::Result<by_axum::axum::Router<crate::AppState>> {
    Ok(axum::Router::new().route("/teams", post(migrate_team_handler)))
}
