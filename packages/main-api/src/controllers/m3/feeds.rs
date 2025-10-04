use crate::models::migrators::post::migrate_posts;
use crate::{AppState, Error2};
use bdk::prelude::*;
use by_axum::axum::extract::State;

pub async fn migrate_posts_handler(
    State(AppState { dynamo, pool, .. }): State<AppState>,
) -> Result<(), Error2> {
    if let Err(e) = migrate_posts(&dynamo.client, &pool, None).await {
        tracing::error!("list_posts_handler: migrate_posts error: {}", e);
    }

    Ok(())
}
