// Migrated from packages/main-api/src/controllers/v3/auth/verification/send_code.rs
use crate::features::auth::models::*;
use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(untagged)]
pub enum SendCodeRequest {
    SMS { phone: String },
    Email { email: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SendCodeResponse {
    pub expired_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SendPasswordResetCodeRequest {
    pub email: String,
}

#[post("/api/auth/verification/send-verification-code")]
pub async fn send_code_handler(req: SendCodeRequest) -> Result<SendCodeResponse> {
    match req {
        SendCodeRequest::Email { email } => send_email_code_handler(email).await,
        SendCodeRequest::SMS { phone } => send_phone_code_handler(phone).await,
    }
}

#[post("/api/auth/verification/send-password-reset-code")]
pub async fn send_password_reset_code_handler(
    req: SendPasswordResetCodeRequest,
) -> Result<SendCodeResponse> {
    send_password_reset_email_code_handler(req.email).await
}

#[cfg(feature = "server")]
async fn send_password_reset_email_code_handler(email: String) -> Result<SendCodeResponse> {
    use crate::features::auth::constants::EXPIRATION_TIME;

    let cli = crate::features::auth::config::get().dynamodb();
    let (users, _) = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1)).await?;

    if users.is_empty() {
        tracing::debug!("Skipping password reset verification email for non-registered email");
        return Ok(SendCodeResponse {
            expired_at: crate::common::utils::time::get_now_timestamp() + EXPIRATION_TIME as i64,
        });
    }

    // Skip the duplicate email check since password reset requires an existing account
    send_email_code(email).await
}

#[cfg(feature = "server")]
pub async fn send_phone_code_handler(phone: String) -> Result<SendCodeResponse> {
    use crate::features::auth::constants::{
        ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT,
    };
    use crate::features::auth::utils::generate_random_numeric_code;

    let cli = crate::features::auth::config::get().dynamodb();
    let sns = crate::features::auth::config::get().sns();

    let (verification_list, _) =
        PhoneVerification::find_by_phone(cli, &phone, PhoneVerification::opt_one()).await?;

    let PhoneVerification {
        value, expired_at, ..
    } = if !verification_list.is_empty()
        && verification_list[0].expired_at > crate::common::utils::time::get_now_timestamp()
        && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
    {
        verification_list[0].clone()
    } else if !verification_list.is_empty()
        && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
        && verification_list[0].expired_at
            < (crate::common::utils::time::get_now_timestamp() - ATTEMPT_BLOCK_TIME)
    {
        return Err(Error::ExceededAttemptPhoneVerification);
    } else {
        let code = generate_random_numeric_code();
        let expired_at = crate::common::utils::time::get_now_timestamp() + EXPIRATION_TIME as i64;

        if verification_list.len() > 0 {
            let mut v = verification_list[0].clone();
            PhoneVerification::updater(v.pk.clone(), v.sk.clone())
                .with_attempt_count(0)
                .with_value(code.clone())
                .with_expired_at(expired_at)
                .execute(cli)
                .await?;
            v.value = code;
            v.expired_at = expired_at;
            v
        } else {
            let phone_verification = PhoneVerification::new(phone.clone(), code, expired_at);
            phone_verification.create(cli).await?;
            phone_verification
        }
    };

    // Send SMS with verification code
    let message = format!(
        "Ratel: {} is your code for login. Don't share your code",
        value
    );
    sns.send_sms(&phone, &message).await?;

    Ok(SendCodeResponse { expired_at })
}

#[cfg(feature = "server")]
pub async fn send_email_code_handler(email: String) -> Result<SendCodeResponse> {
    let cli = crate::features::auth::config::get().dynamodb();

    // Check if email is already registered (signup only)
    let (existing_users, _) =
        User::find_by_email(cli, &email, UserQueryOption::builder().limit(1)).await?;
    if !existing_users.is_empty() {
        return Err(Error::Duplicate(format!(
            "Email already registered: {}. Please use Forgot Password instead.",
            email
        )));
    }

    send_email_code(email).await
}

/// Sends an email verification code without checking for existing accounts.
/// Used by both signup (via send_email_code_handler with duplicate check)
/// and password reset (via send_password_reset_email_code_handler).
#[cfg(feature = "server")]
async fn send_email_code(email: String) -> Result<SendCodeResponse> {
    use crate::common::models::notification::Notification;
    use crate::features::auth::constants::{
        ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT,
    };
    use crate::features::auth::utils::generate_random_code;

    let cli = crate::features::auth::config::get().dynamodb();

    let (verification_list, _) = EmailVerification::find_by_email(
        cli,
        &email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await?;

    let EmailVerification {
        value, expired_at, ..
    } = if !verification_list.is_empty()
        && verification_list[0].expired_at > crate::common::utils::time::get_now_timestamp()
        && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
    {
        verification_list[0].clone()
    } else if !verification_list.is_empty()
        && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
        && verification_list[0].expired_at
            < (crate::common::utils::time::get_now_timestamp() - ATTEMPT_BLOCK_TIME)
    {
        return Err(Error::ExceededAttemptEmailVerification);
    } else {
        let code = generate_random_code();
        let expired_at = crate::common::utils::time::get_now_timestamp() + EXPIRATION_TIME as i64;

        if verification_list.len() > 0 {
            let mut v = verification_list[0].clone();
            EmailVerification::updater(v.pk.clone(), v.sk.clone())
                .with_attempt_count(0)
                .with_value(code.clone())
                .with_expired_at(expired_at)
                .execute(cli)
                .await?;
            v.value = code;
            v.expired_at = expired_at;
            v
        } else {
            let email_verification = EmailVerification::new(email.clone(), code, expired_at);
            email_verification.create(cli).await?;
            email_verification
        }
    };

    // Create a Notification document instead of directly calling SES.
    // DynamoDB Streams will trigger a Lambda function to send the email.
    let notification = Notification::new(NotificationData::SendVerificationCode {
        code: value,
        email: email.clone(),
    });
    notification.create(cli).await?;

    Ok(SendCodeResponse { expired_at })
}
