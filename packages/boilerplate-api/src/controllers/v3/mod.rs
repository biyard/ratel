pub mod examples;

use crate::*;

pub fn route() -> Result<Router> {
    Ok(Router::new().nest("/examples", examples::route()?))
}
