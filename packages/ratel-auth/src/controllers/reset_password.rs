use crate::models::{EmailVerification, EmailVerificationQueryOption};
use crate::utils::password::hash_password;

use common::models::user::UserQueryOption;
use common::models::*;
use common::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub password: String,
    pub code: String,
}

#[post("/api/auth/reset")]
pub async fn reset_password_handler(
    form: dioxus::fullstack::Form<ResetPasswordRequest>,
) -> std::result::Result<User, ServerFnError> {
    let c = crate::config::get();
    let cli = c.common.dynamodb();
    let req: ResetPasswordRequest = form.0;

    let email = req.email;
    let code = req.code;
    let password = req.password;

    let is_invalid = EmailVerification::find_by_email_and_code(
        cli,
        email.clone(),
        EmailVerificationQueryOption::builder()
            .sk(code.clone())
            .limit(1),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?
    .0
    .is_empty();

    #[cfg(feature = "bypass")]
    let is_invalid = is_invalid && !code.eq("000000");

    if is_invalid {
        return Err(ServerFnError::new("Invalid verification code"));
    }

    let (users, _) = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1))
        .await
        .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    if users.is_empty() {
        return Err(ServerFnError::new(format!(
            "Not registered email: {}",
            email
        )));
    }

    let user = users[0].clone();
    let hashed_password = hash_password(&password);

    let user: User = User::updater(user.pk, user.sk)
        .with_password(hashed_password)
        .execute(cli)
        .await
        .map_err(|e| ServerFnError::new(format!("DB update failed: {:?}", e)))?;

    Ok(user)
}
