pub mod find_user;

#[cfg(test)]
pub mod tests;

use find_user::find_user_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().route("/", get(find_user_handler)))
}
