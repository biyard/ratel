use crate::{
    AppState, Error,
    models::{EmailVerification, EmailVerificationQueryOption, user::User},
};
use bdk::prelude::*;

use crate::models::UserQueryOption;
use crate::utils::password::hash_password;
use aide::NoApi;
use by_axum::axum::extract::{Json, State};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub password: String,
    pub code: String,
}

pub async fn reset_password_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<Json<User>, Error> {
    let email = req.email;
    let code = req.code;
    let password = req.password;
    let is_invalid = EmailVerification::find_by_email_and_code(
        &dynamo.client,
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
        User::find_by_email(&dynamo.client, &email, UserQueryOption::builder().limit(1)).await?;
    if users.len() == 0 {
        return Err(Error::NotFound(format!("Not Registered Email: {}", email)));
    }
    let user = users[0].clone();
    let hashed_password = hash_password(&password);

    let user = User::updater(user.pk, user.sk)
        .with_password(hashed_password)
        .execute(&dynamo.client)
        .await?;

    Ok(Json(user))
}
