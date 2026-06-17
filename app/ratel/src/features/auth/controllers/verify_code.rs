// Migrated from packages/main-api/src/controllers/v3/auth/verification/verify_code.rs
use crate::features::auth::models::*;
use crate::features::auth::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
#[serde(untagged)]
pub enum VerifyCodeRequest {
    Email {
        email: String,
        code: String,
    },
    Phone {
        phone: String,
        code: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct VerifyCodeResponse {
    pub success: bool,
}

#[post("/api/auth/verification/verify-code")]
pub async fn verify_code_handler(req: VerifyCodeRequest) -> Result<VerifyCodeResponse> {
    match req {
        VerifyCodeRequest::Email { email, code } => {
            verify_email_code_handler(email, code).await
        }
        VerifyCodeRequest::Phone { phone, code } => {
            verify_phone_code_handler(phone, code).await
        }
    }
}

/// Shared email-code verification used by verify-code, login, and signup.
/// Returns Ok(()) when the code is valid; increments attempt_count and errors
/// otherwise. Honors the `bypass` test code "000000". Does NOT consume the
/// code (so login→signup can reuse the same code within its window).
#[cfg(feature = "server")]
pub async fn verify_email_code(
    cli: &aws_sdk_dynamodb::Client,
    email: &str,
    code: &str,
) -> Result<()> {
    use crate::features::auth::constants::MAX_ATTEMPT_COUNT;

    let now = crate::common::utils::time::get_now_timestamp();
    let (verification_list, _) = EmailVerification::find_by_email(
        cli,
        email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await?;

    // Honor the test bypass code BEFORE requiring a stored verification
    // record. Test setup logs in without first calling send-verification-code,
    // so there is no EmailVerification row — the bypass must short-circuit
    // here, not after the emptiness check. `bypass` is test/local only and is
    // never compiled into production, so real logins still require a record.
    #[cfg(feature = "bypass")]
    if code.eq("000000") {
        return Ok(());
    }

    if verification_list.is_empty() {
        return Err(Error::NotFoundVerificationCode);
    }

    let email_verification = verification_list[0].clone();

    if email_verification.attempt_count >= MAX_ATTEMPT_COUNT {
        return Err(Error::ExceededAttemptEmailVerification);
    }
    if email_verification.expired_at < now {
        return Err(Error::ExpiredVerification);
    }
    if email_verification.value != code {
        EmailVerification::updater(email_verification.pk, email_verification.sk)
            .increase_attempt_count(1)
            .execute(cli)
            .await?;
        return Err(Error::InvalidVerificationCode);
    }

    Ok(())
}

#[cfg(feature = "server")]
async fn verify_email_code_handler(email: String, code: String) -> Result<VerifyCodeResponse> {
    verify_email_code(crate::features::auth::config::get().dynamodb(), &email, &code).await?;
    Ok(VerifyCodeResponse { success: true })
}

#[cfg(feature = "server")]
async fn verify_phone_code_handler(
    phone: String,
    code: String,
) -> Result<VerifyCodeResponse> {
    use crate::features::auth::constants::MAX_ATTEMPT_COUNT;

    let cli = crate::features::auth::config::get().dynamodb();
    let now = crate::common::utils::time::get_now_timestamp();
    let (verification_list, _) = PhoneVerification::find_by_phone(
        cli,
        &phone,
        PhoneVerificationQueryOption::builder().limit(1),
    )
    .await?;

    if verification_list.is_empty() {
        return Err(Error::NotFoundVerificationCode);
    }

    tracing::debug!("code {}", code);

    #[cfg(feature = "bypass")]
    if code.eq("000000") {
        return Ok(VerifyCodeResponse { success: true });
    }

    let phone_verification = verification_list[0].clone();

    if phone_verification.attempt_count >= MAX_ATTEMPT_COUNT {
        return Err(Error::ExceededAttemptPhoneVerification);
    }

    if phone_verification.expired_at < now {
        return Err(Error::ExpiredVerification);
    }

    if phone_verification.value != code {
        PhoneVerification::updater(phone_verification.pk, phone_verification.sk)
            .increase_attempt_count(1)
            .execute(cli)
            .await?;
        return Err(Error::InvalidVerificationCode);
    }

    Ok(VerifyCodeResponse { success: true })
}
