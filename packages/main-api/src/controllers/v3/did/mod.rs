pub mod reconcil_identity_verification;

use reconcil_identity_verification::reconcil_identity_verification_handler;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().route("/kyc", post(reconcil_identity_verification_handler)))
}
