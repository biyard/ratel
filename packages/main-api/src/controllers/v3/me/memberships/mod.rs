mod change_membership;
mod history;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", post(change_membership::change_membership_handler))
        .route("/history", get(history::get_purchase_history_handler)))
}
