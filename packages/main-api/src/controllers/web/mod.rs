pub mod home_page;

use crate::error::Error;
use crate::utils::aws::DynamoClient;
use axum::Router;
use axum::native_routing::get_service;
use bdk::prelude::*;
use home_page::home_page_handler;
use tower_http::services::ServeDir;

pub fn route() -> Result<Router, Error> {
    let DynamoClient { client } = DynamoClient::new(None);

    Ok(Router::new()
        .native_route("/", axum::native_routing::get(home_page_handler))
        .nest_service("/assets", get_service(ServeDir::new("dist/assets")))
        .with_state(client))
}
