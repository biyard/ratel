use crate::{
    Error,
    controllers::m3::memberships::*,
    route_v3::AppState,
    utils::aws::{DynamoClient, SesClient},
};
use bdk::prelude::*;
use by_axum::{aide::axum::routing::*, axum::Router};

pub struct RouteDeps {
    pub dynamo_client: DynamoClient,
    pub ses_client: SesClient,
    pub pool: bdk::prelude::sqlx::PgPool,
}

pub fn route(
    RouteDeps {
        dynamo_client,
        pool,
        ses_client,
    }: RouteDeps,
) -> Result<Router, Error> {
    let app_state = AppState {
        dynamo: dynamo_client,
        ses: ses_client,
        pool,
    };

    Ok(Router::new().nest(
        "/memberships",
        Router::new()
            .route(
                "/",
                post(create_membership_handler).get(list_memberships_handler),
            )
            .route(
                "/:membership_id",
                get(get_membership_handler)
                    .patch(update_membership_handler)
                    .delete(delete_membership_handler),
            )
            .with_state(app_state),
    ))
}
