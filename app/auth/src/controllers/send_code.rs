// Migrated from packages/main-api/src/controllers/v3/auth/verification/send_code.rs
use crate::models::*;
use crate::*;

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

#[post("/api/auth/verification/send-verification-code")]
pub async fn send_code_handler(req: SendCodeRequest) -> Result<SendCodeResponse> {
    match req {
        SendCodeRequest::Email { email } => send_email_code_handler(email).await,
        SendCodeRequest::SMS { phone } => send_phone_code_handler(phone).await,
    }
}

#[cfg(feature = "server")]
pub async fn send_phone_code_handler(
    phone: String,
) -> Result<SendCodeResponse> {
    use crate::constants::{ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT};
    use crate::utils::generate_random_numeric_code;

    let cli = crate::config::get().dynamodb();
    let sns = crate::config::get().sns();

    let (verification_list, _) =
        PhoneVerification::find_by_phone(cli, &phone, PhoneVerification::opt_one())
            .await?;

    let PhoneVerification {
        value, expired_at, ..
    } = if !verification_list.is_empty()
        && verification_list[0].expired_at > common::utils::time::get_now_timestamp()
        && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
    {
        verification_list[0].clone()
    } else if !verification_list.is_empty()
        && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
        && verification_list[0].expired_at < (common::utils::time::get_now_timestamp() - ATTEMPT_BLOCK_TIME)
    {
        return Err(Error::ExceededAttemptPhoneVerification);
    } else {
        let code = generate_random_numeric_code();
        let expired_at = common::utils::time::get_now_timestamp() + EXPIRATION_TIME as i64;

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
pub async fn send_email_code_handler(
    email: String,
) -> Result<SendCodeResponse> {
    use crate::constants::{ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT};
    use crate::types::email_operation::EmailOperation;
    use crate::utils::generate_random_code;

    let cli = crate::config::get().dynamodb();
    let ses = crate::config::get().ses();

    let (users, _) = User::find_by_email(cli, &email, User::opt_one()).await?;
    if users.is_empty() {
        return Err(Error::UserNotRegistered);
    }

    let (verification_list, _) = EmailVerification::find_by_email(
        cli,
        &email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await?;

    let EmailVerification {
        value, expired_at, ..
    } = if !verification_list.is_empty()
        && verification_list[0].expired_at > common::utils::time::get_now_timestamp()
        && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
    {
        verification_list[0].clone()
    } else if !verification_list.is_empty()
        && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
        && verification_list[0].expired_at < (common::utils::time::get_now_timestamp() - ATTEMPT_BLOCK_TIME)
    {
        return Err(Error::ExceededAttemptEmailVerification);
    } else {
        let code = generate_random_code();
        let expired_at = common::utils::time::get_now_timestamp() + EXPIRATION_TIME as i64;

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

    let user_email = email.clone();
    let display_name = user_email.clone();

    let mut chars = value.chars();
    let code_1 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_2 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_3 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_4 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_5 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_6 = chars.next().map(|c| c.to_string()).unwrap_or_default();

    let email_template = EmailTemplate {
        targets: vec![user_email.clone()],
        operation: EmailOperation::SignupSecurityCode {
            display_name,
            code_1,
            code_2,
            code_3,
            code_4,
            code_5,
            code_6,
        },
    };

    email_template.send_email(ses).await?;

    Ok(SendCodeResponse { expired_at })
}
