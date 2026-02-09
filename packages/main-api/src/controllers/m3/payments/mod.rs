pub mod cancel_payment;
pub mod dto;
pub mod list_payments;

#[cfg(test)]
pub mod tests;

pub use cancel_payment::*;
pub use dto::*;
pub use list_payments::*;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/", get(list_all_payments_handler))
        .route("/:payment_id/cancel", post(cancel_payment_handler)))
}
