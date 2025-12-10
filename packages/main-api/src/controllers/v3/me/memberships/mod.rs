mod change_membership;
mod history;

#[cfg(test)]
pub mod tests;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", post(change_membership::change_membership_handler))
        .route("/history", get(history::get_purchase_history_handler)))
}
