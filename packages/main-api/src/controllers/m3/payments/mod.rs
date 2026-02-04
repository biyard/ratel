pub mod dto;
pub mod list_payments;

#[cfg(test)]
pub mod tests;

pub use dto::*;
pub use list_payments::*;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().route("/", get(list_all_payments_handler)))
}
