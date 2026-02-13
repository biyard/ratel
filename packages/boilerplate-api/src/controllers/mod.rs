pub mod v3;

use crate::*;

pub fn route() -> Result<Router> {
    Ok(Router::new().nest("/v3", v3::route()?))
}
