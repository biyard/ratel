pub mod login;
pub mod logout;
pub mod reset_password;
pub mod signup;

#[cfg(test)]
pub mod tests;

pub mod verification {
    pub mod send_code;
    pub mod verify_code;

    #[cfg(test)]
    pub mod tests;
}

use login::login_handler;
use logout::logout_handler;
use reset_password::reset_password_handler;
use signup::signup_handler;
use verification::{send_code::send_code_handler, verify_code::verify_code_handler};

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/signup", post(signup_handler))
        .route("/reset", post(reset_password_handler))
        .nest(
            "/verification",
            Router::new()
                .route("/send-verification-code", post(send_code_handler))
                .route("/verify-code", post(verify_code_handler)),
        ))
}
