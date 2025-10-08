use crate::error::Error;
use axum::Router;
use bdk::prelude::*;

pub fn route() -> Result<Router, Error> {
    Ok(Router::new().native_route("/", axum::native_routing::get(|| async { "Hello, World!" })))
}
