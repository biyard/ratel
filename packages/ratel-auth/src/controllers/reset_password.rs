// Migrated from packages/main-api/src/controllers/v3/auth/reset_password.rs
use crate::models::*;
#[cfg(feature = "server")]
use crate::utils::password::hash_password;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ResetPasswordRequest {
    pub email: String,
    pub password: String,
    pub code: String,
}

#[post("/api/auth/reset")]
pub async fn reset_password_handler(req: ResetPasswordRequest) -> Result<User> {
    let cli = crate::config::get().dynamodb();

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
    .await?
    .0
    .len()
        == 0;

    #[cfg(feature = "bypass")]
    let is_invalid = is_invalid && !code.eq("000000");

    if is_invalid {
        return Err(Error::InvalidVerificationCode);
    }

    let (users, _) =
        User::find_by_email(cli, &email, UserQueryOption::builder().limit(1)).await?;
    if users.len() == 0 {
        return Err(Error::NotFound(format!("Not Registered Email: {}", email)));
    }
    let user = users[0].clone();
    let hashed_password = hash_password(&password);

    let user = User::updater(user.pk, user.sk)
        .with_password(hashed_password)
        .execute(cli)
        .await?;

    Ok(user)
}
