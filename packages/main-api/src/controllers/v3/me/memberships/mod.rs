pub mod change_membership;
mod get_membership;
mod history;

#[cfg(test)]
pub mod tests;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", get(get_membership::get_membership_handler))
        .route("/", post(change_membership::change_membership_handler))
        .route("/history", get(history::get_purchase_history_handler)))
}
