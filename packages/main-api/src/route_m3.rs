use crate::{AppState, Error2, RouteDeps, controllers::m3::feeds::migrate_posts_handler};
use bdk::prelude::*;
use by_axum::axum::Router;
use dto::axum::native_routing::{self};

pub fn route(
    RouteDeps {
        dynamo_client,
        pool,
        ses_client,
    }: RouteDeps,
) -> Result<Router, Error2> {
    Ok(Router::new().native_route(
        "/feeds",
        native_routing::post(migrate_posts_handler).with_state(AppState {
            dynamo: dynamo_client,
            ses: ses_client,
            pool,
        }),
    ))
}
