pub mod change_team_membership;
pub mod get_team_membership;
pub mod get_team_membership_history;

#[cfg(test)]
pub mod tests;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    use change_team_membership::change_team_membership_handler;
    use get_team_membership::get_team_membership_handler;
    use get_team_membership_history::get_team_purchase_history_handler;

    Ok(Router::new()
        .route(
            "/",
            get(get_team_membership_handler).post(change_team_membership_handler),
        )
        .route("/history", get(get_team_purchase_history_handler)))
}
