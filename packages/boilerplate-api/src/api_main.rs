use bdk::prelude::{by_axum::axum::Router, *};

use crate::{config, controllers};

pub async fn api_main() -> Result<Router, crate::Error> {
    let app = by_axum::new();
    let _conf = config::get();

    Ok(app.merge(controllers::route()?))
}
