mod identification;
mod pay_with_billing_key;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/identify", post(identification::identification_handler))
        .native_route(
            "/memberships",
            native_routing::post(pay_with_billing_key::pay_with_billing_key_handler),
        ))
}
