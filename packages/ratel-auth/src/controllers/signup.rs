use crate::constants::SESSION_KEY_USER_ID;
use crate::models::{
    EmailVerification, EmailVerificationQueryOption, PhoneVerification,
    PhoneVerificationQueryOption, UserEvmAddress, UserReferralCode, UserRefreshToken,
};
use crate::types::Provider;
use crate::utils::password::hash_password;
use crate::utils::referral_code::generate_referral_code;
use crate::utils::user_factory::{new_phone_user, new_user};
use crate::utils::validator::{validate_image_url, validate_username};

use common::models::user::UserQueryOption;
use common::models::*;
use common::*;
use dioxus::prelude::*;
use ::validator::Validate;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, Validate)]
pub struct SignupRequest {
    #[serde(flatten)]
    pub signup_type: SignupType,
    pub display_name: String,
    #[validate(custom(function = "validate_username"))]
    pub username: String,
    #[validate(custom(function = "validate_image_url"))]
    pub profile_url: String,
    pub description: String,
    pub term_agreed: bool,
    pub informed_agreed: bool,

    pub evm_address: Option<String>,
    pub phone_number: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum SignupType {
    Email {
        email: String,
        password: String,
        code: String,
    },
    Phone {
        phone: String,
        code: String,
    },
    OAuth {
        provider: Provider,
        access_token: String,
    },
    Telegram {
        telegram_raw: String,
    },
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignupResponse {
    #[serde(flatten)]
    pub user: User,
    pub refresh_token: Option<String>,
}

#[post("/api/auth/signup", session: Extension<TowerSession>)]
pub async fn signup_handler(
    form: dioxus::fullstack::Form<SignupRequest>,
) -> std::result::Result<SignupResponse, ServerFnError> {
    let c = crate::config::get();
    let cli = c.common.dynamodb();
    let req: SignupRequest = form.0;

    req.validate()
        .map_err(|e| ServerFnError::new(format!("Invalid input: {}", e)))?;

    let evm_address = req.evm_address.clone();
    let user = match req.signup_type.clone() {
        SignupType::Email {
            email,
            password,
            code,
        } => signup_with_email_password(cli, req.clone(), email, password, code).await?,
        SignupType::Phone { phone, code } => signup_with_phone(cli, phone, code).await?,
        SignupType::OAuth {
            provider,
            access_token,
        } => signup_with_oauth(cli, req.clone(), provider, access_token).await?,
        SignupType::Telegram { .. } => {
            return Err(ServerFnError::new("Telegram signup not yet implemented"));
        }
    };

    if let Some(evm_address) = evm_address {
        UserEvmAddress::new(user.pk.clone(), evm_address)
            .create(cli)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to create EVM address: {:?}", e)))?;
    }

    UserReferralCode::new(user.pk.clone(), generate_referral_code())
        .create(cli)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create referral code: {:?}", e)))?;

    session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await
        .map_err(|e| ServerFnError::new(format!("Session insert failed: {:?}", e)))?;

    let device_id: Option<String> = req.device_id.clone();

    let refresh_token = if let Some(device_id) = device_id {
        let (rt, plain) = UserRefreshToken::new(&user, device_id);
        rt.upsert(cli)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to upsert refresh token: {:?}", e)))?;
        Some(plain)
    } else {
        None
    };

    Ok(SignupResponse {
        user,
        refresh_token,
    })
}

async fn signup_with_email_password(
    cli: &aws_sdk_dynamodb::Client,
    SignupRequest {
        display_name,
        username,
        profile_url,
        term_agreed,
        informed_agreed,
        ..
    }: SignupRequest,
    email: String,
    password: String,
    code: String,
) -> std::result::Result<User, ServerFnError> {
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
    if !users.is_empty() {
        return Err(ServerFnError::new(format!(
            "Email already registered: {}",
            email
        )));
    }
    let hashed_password = hash_password(&password);

    let user = new_user(
        display_name,
        email,
        profile_url,
        term_agreed,
        informed_agreed,
        UserType::Individual,
        username,
        Some(hashed_password),
    );

    user.create(cli)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create user: {:?}", e)))?;

    Ok(user)
}

async fn signup_with_oauth(
    cli: &aws_sdk_dynamodb::Client,
    SignupRequest {
        display_name,
        username,
        profile_url,
        term_agreed,
        informed_agreed,
        ..
    }: SignupRequest,
    provider: Provider,
    access_token: String,
) -> std::result::Result<User, ServerFnError> {
    let email = provider
        .get_email(&access_token)
        .await
        .map_err(|e| ServerFnError::new(format!("OAuth verification failed: {:?}", e)))?;

    let (users, _) = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1))
        .await
        .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;
    if !users.is_empty() {
        return Err(ServerFnError::new(format!(
            "Email already registered: {}",
            email
        )));
    }

    let user = new_user(
        display_name,
        email,
        profile_url,
        term_agreed,
        informed_agreed,
        UserType::Individual,
        username,
        None,
    );

    user.create(cli)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create user: {:?}", e)))?;

    Ok(user)
}

async fn signup_with_phone(
    cli: &aws_sdk_dynamodb::Client,
    phone: String,
    code: String,
) -> std::result::Result<User, ServerFnError> {
    let is_invalid = PhoneVerification::find_by_phone_and_code(
        cli,
        phone.clone(),
        PhoneVerificationQueryOption::builder()
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

    let (users, _) = User::find_by_phone(cli, &phone, UserQueryOption::builder().limit(1))
        .await
        .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    if !users.is_empty() {
        return Ok(users[0].clone());
    }

    let user = new_phone_user(phone);

    user.create(cli)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to create user: {:?}", e)))?;

    Ok(user)
}
