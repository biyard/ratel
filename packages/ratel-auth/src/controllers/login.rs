use crate::constants::SESSION_KEY_USER_ID;
use crate::models::{
    PhoneVerification, PhoneVerificationQueryOption, UserPhoneNumber, UserRefreshToken,
    UserTelegram, UserTelegramQueryOption,
};
use crate::transact_write;
use crate::types::Provider;
use crate::utils::password::hash_password;
use crate::utils::telegram_auth::parse_telegram_raw;
use crate::utils::time::get_now_timestamp;
use crate::utils::user_factory::new_user;

use common::models::user::UserQueryOption;
use common::models::*;
use common::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum LoginRequest {
    Phone {
        phone: String,
        code: String,
        device_id: Option<String>,
    },
    Email {
        email: String,
        password: String,
        device_id: Option<String>,
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
pub struct LoginResponse {
    #[serde(flatten)]
    pub user: User,
    pub refresh_token: Option<String>,
}

#[post("/api/auth/login", session: Extension<TowerSession>)]
pub async fn login_handler(
    form: dioxus::fullstack::Form<LoginRequest>,
) -> std::result::Result<LoginResponse, ServerFnError> {
    let c = crate::config::get();
    let cli = c.common.dynamodb();
    let req: LoginRequest = form.0;

    let user = match req.clone() {
        LoginRequest::Phone { phone, code, .. } => login_with_phone(cli, phone, code).await?,
        LoginRequest::Email {
            email, password, ..
        } => login_with_email(cli, email, password).await?,
        LoginRequest::OAuth {
            provider,
            access_token,
        } => login_with_oauth(cli, provider, access_token).await?,
        LoginRequest::Telegram { telegram_raw } => {
            login_with_telegram(cli, telegram_raw).await?
        }
    };

    session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await
        .map_err(|e| ServerFnError::new(format!("Session insert failed: {:?}", e)))?;

    let device_id: Option<String> = match &req {
        LoginRequest::Phone { device_id, .. } => device_id.clone(),
        LoginRequest::Email { device_id, .. } => device_id.clone(),
        _ => None,
    };

    let refresh_token = if let Some(device_id) = device_id {
        let (rt, plain) = UserRefreshToken::new(&user, device_id);
        rt.upsert(cli)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to upsert refresh token: {:?}", e)))?;
        Some(plain)
    } else {
        None
    };

    Ok(LoginResponse {
        user,
        refresh_token,
    })
}

async fn login_with_phone(
    cli: &aws_sdk_dynamodb::Client,
    phone: String,
    code: String,
) -> std::result::Result<User, ServerFnError> {
    let now = get_now_timestamp();
    let (verification_list, _) = PhoneVerification::find_by_phone(
        cli,
        &phone,
        PhoneVerificationQueryOption::builder().limit(1),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    if verification_list.is_empty() {
        return Err(ServerFnError::new("Verification code not found"));
    }

    #[cfg(feature = "bypass")]
    if !code.eq("000000") {
        verify_phone_code(&verification_list[0], &code, now, cli).await?;
    }

    #[cfg(not(feature = "bypass"))]
    verify_phone_code(&verification_list[0], &code, now, cli).await?;

    let (res, _) =
        UserPhoneNumber::find_by_phone_number(cli, &phone, UserPhoneNumber::opt_one())
            .await
            .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    let user = if res.is_empty() {
        let username = format!("user{}", now);
        let display_name = phone.clone();
        let email = format!("{}@phone.placeholder", username);

        let user = new_user(
            display_name,
            email,
            String::new(),
            false,
            false,
            UserType::Anonymous,
            username,
            None,
        );

        let user_phone = UserPhoneNumber::new(user.pk.clone(), phone.clone());

        transact_write!(
            cli,
            user.create_transact_write_item(),
            user_phone.create_transact_write_item()
        )
        .map_err(|e| ServerFnError::new(format!("Transaction failed: {:?}", e)))?;

        user
    } else {
        let user_phone = res.first().cloned().ok_or_else(|| {
            ServerFnError::new("No user linked with the given phone number")
        })?;
        User::get(cli, user_phone.pk, Some(EntityType::User))
            .await
            .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?
            .ok_or_else(|| ServerFnError::new("No user linked with the given phone number"))?
    };

    Ok(user)
}

async fn verify_phone_code(
    phone_verification: &PhoneVerification,
    code: &str,
    now: i64,
    cli: &aws_sdk_dynamodb::Client,
) -> std::result::Result<(), ServerFnError> {
    if phone_verification.expired_at < now {
        return Err(ServerFnError::new("Verification code expired"));
    }

    if phone_verification.value != code {
        PhoneVerification::updater(
            phone_verification.pk.clone(),
            phone_verification.sk.clone(),
        )
        .increase_attempt_count(1)
        .execute(cli)
        .await
        .map_err(|e| ServerFnError::new(format!("DB update failed: {:?}", e)))?;
        return Err(ServerFnError::new("Invalid verification code"));
    }

    Ok(())
}

async fn login_with_oauth(
    cli: &aws_sdk_dynamodb::Client,
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

    let user = users
        .into_iter()
        .next()
        .ok_or_else(|| ServerFnError::new("No user found with the given email"))?;

    Ok(user)
}

async fn login_with_email(
    cli: &aws_sdk_dynamodb::Client,
    email: String,
    password: String,
) -> std::result::Result<User, ServerFnError> {
    let hashed_password = hash_password(&password);
    let (u, _) = User::find_by_email_and_password(
        cli,
        &email,
        UserQueryOption::builder().sk(hashed_password),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    let user = u
        .into_iter()
        .next()
        .ok_or_else(|| ServerFnError::new("Invalid email or password"))?;

    Ok(user)
}

async fn login_with_telegram(
    cli: &aws_sdk_dynamodb::Client,
    telegram_raw: String,
) -> std::result::Result<User, ServerFnError> {
    let telegram_user = parse_telegram_raw(telegram_raw.clone(), "")
        .map_err(|e| ServerFnError::new(format!("Invalid telegram data: {}", e)))?;

    let (res, _) = UserTelegram::find_by_telegram_id(
        cli,
        telegram_user.id,
        UserTelegramQueryOption::builder().limit(1),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    let user = if res.is_empty() {
        let username = telegram_user
            .username
            .clone()
            .unwrap_or(format!("telegram{}", telegram_user.id))
            .to_lowercase();
        let display_name = format!(
            "{} {}",
            telegram_user.first_name.unwrap_or_default(),
            telegram_user.last_name.unwrap_or_default()
        );
        let email = format!("{}@telegram.placeholder", username);
        let user = new_user(
            display_name,
            email,
            telegram_user.photo_url.unwrap_or_default(),
            false,
            false,
            UserType::Anonymous,
            username,
            None,
        );
        let user_telegram =
            UserTelegram::new(user.pk.clone(), telegram_user.id, telegram_raw);

        transact_write!(
            cli,
            user.create_transact_write_item(),
            user_telegram.create_transact_write_item()
        )
        .map_err(|e| ServerFnError::new(format!("Transaction failed: {:?}", e)))?;

        user
    } else {
        let user_telegram = res.first().cloned().ok_or_else(|| {
            ServerFnError::new("No user linked with the given telegram account")
        })?;
        User::get(cli, user_telegram.pk, Some(EntityType::User))
            .await
            .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?
            .ok_or_else(|| {
                ServerFnError::new("No user linked with the given telegram account")
            })?
    };

    Ok(user)
}
