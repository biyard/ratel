mod get_attributes;
mod get_or_create_did;
mod sign_attributes;

use get_or_create_did::get_or_create_did_handler;

use crate::features::did::*;
use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .native_route(
            "/",
            nr::get(get_or_create_did_handler).put(sign_attributes::sign_attributes_handler),
        )
        .native_route(
            "/attributes",
            nr::get(get_attributes::get_attributes_handler),
        ))
}
