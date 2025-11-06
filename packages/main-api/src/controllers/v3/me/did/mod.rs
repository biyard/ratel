mod get_or_create_did;
mod sign_attributes;

use get_or_create_did::get_or_create_did_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().native_route(
        "/",
        nr::get(get_or_create_did_handler).patch(sign_attributes::sign_attributes_handler),
    ))
}
