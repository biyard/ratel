mod identification;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().route("/identify", post(identification::identification_handler)))
}
