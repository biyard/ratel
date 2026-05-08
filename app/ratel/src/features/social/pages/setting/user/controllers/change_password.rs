use super::super::*;
use crate::features::social::types::SocialError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ChangePasswordResponse {
    pub success: bool,
}

#[post("/api/me/password", user: crate::features::auth::User)]
pub async fn change_password_handler(body: ChangePasswordRequest) -> Result<ChangePasswordResponse> {
    use crate::common::utils::password::hash_password;

    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let stored_password = user.password.clone().unwrap_or_default();
    if stored_password.is_empty() {
        return Err(SocialError::IncorrectCurrentPassword.into());
    }

    let current_hashed = hash_password(&body.current_password);
    if current_hashed != stored_password {
        return Err(SocialError::IncorrectCurrentPassword.into());
    }

    if body.new_password.len() < 8 {
        return Err(SocialError::PasswordTooShort.into());
    }

    let new_hashed = hash_password(&body.new_password);
    crate::features::auth::User::updater(user.pk.clone(), user.sk.clone())
        .with_password(new_hashed)
        .execute(cli)
        .await?;

    Ok(ChangePasswordResponse { success: true })
}
