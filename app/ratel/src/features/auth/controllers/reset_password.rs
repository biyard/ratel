// Backward-compat: password reset for already-shipped mobile apps (the Android
// build in App Store review). New passwordless clients never call this.
use crate::features::auth::models::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ResetPasswordRequest {
    pub email: String,
    pub password: String,
    pub code: String,
}

#[post("/api/auth/reset")]
pub async fn reset_password_handler(req: ResetPasswordRequest) -> Result<User> {
    let cli = crate::features::auth::config::get().dynamodb();

    let email = req.email;
    let code = req.code;
    let password = req.password;

    crate::features::auth::controllers::verify_code::verify_email_code(cli, &email, &code).await?;

    let (users, _) = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1)).await?;
    let user = users
        .into_iter()
        .next()
        .ok_or(Error::NotFound(format!("Not Registered Email: {}", email)))?;

    let hashed_password = crate::common::utils::password::hash_password(&password);

    let user = User::updater(user.pk, user.sk)
        .with_password(hashed_password)
        .execute(cli)
        .await?;

    Ok(user)
}
