use crate::models::{
    PhoneVerification, PhoneVerificationQueryOption, UserPhoneNumber, UserQueryOption,
    UserRefreshToken, UserTelegram, UserTelegramQueryOption,
};
use crate::time::get_now_timestamp;
use crate::utils::password::hash_password;
use crate::utils::telegram::parse_telegram_raw;
use crate::*;

use bdk::prelude::*;

use by_axum::axum::{
    Extension,
    extract::{Json, State},
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
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

#[derive(
    Debug, Default, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema,
)]
pub struct LoginResponse {
    #[serde(flatten)]
    pub user: User,
    pub refresh_token: Option<String>,
}

pub async fn login_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let user = match req.clone() {
        LoginRequest::Phone {
            phone,
            code,
            device_id: _,
        } => login_with_phone(&dynamo.client, phone, code).await?,
        LoginRequest::Email {
            email,
            password,
            device_id: _,
        } => login_with_email(&dynamo.client, email, password).await?,
        LoginRequest::OAuth {
            provider,
            access_token,
        } => login_with_oauth(&dynamo.client, provider, access_token).await?,
        LoginRequest::Telegram { telegram_raw } => {
            login_with_telegram(&dynamo.client, telegram_raw).await?
        }
    };

    session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await?;

    let device_id: Option<String> = match &req.clone() {
        LoginRequest::Phone { device_id, .. } => device_id.clone(),
        LoginRequest::Email { device_id, .. } => device_id.clone(),
        _ => None,
    };

    let refresh_token = if let Some(device_id) = device_id {
        let (rt, plain) = UserRefreshToken::new(&user, device_id);
        rt.upsert(&dynamo.client).await?;

        Some(plain)
    } else {
        None
    };

    Ok(Json(LoginResponse {
        user,
        refresh_token,
    }))
}

pub async fn login_with_phone(
    cli: &aws_sdk_dynamodb::Client,
    phone: String,
    code: String,
) -> Result<User> {
    // Verify the phone verification code
    let now = get_now_timestamp();
    let (verification_list, _) = PhoneVerification::find_by_phone(
        cli,
        &phone,
        PhoneVerificationQueryOption::builder().limit(1),
    )
    .await?;

    if verification_list.is_empty() {
        return Err(Error::NotFoundVerificationCode);
    }

    #[cfg(feature = "bypass")]
    if code.eq("000000") {
        // Bypass verification for testing
    } else {
        let phone_verification = verification_list[0].clone();

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
    }

    #[cfg(not(feature = "bypass"))]
    {
        let phone_verification = verification_list[0].clone();

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
    }

    // Get or create user by phone number
    let (res, _) =
        UserPhoneNumber::find_by_phone_number(cli, &phone, UserPhoneNumber::opt_one()).await?;

    let user = if res.is_empty() {
        // Create new user with phone number
        let username = format!("user{}", now);
        let display_name = phone.clone();
        let email = format!("{}@phone.placeholder", username);

        let user = User::new(
            display_name,
            email,
            String::new(), // No avatar
            false,         // Not verified
            false,         // Not admin
            crate::types::UserType::Anonymous,
            username,
            None, // No password
        );

        let user_phone = UserPhoneNumber::new(user.pk.clone(), phone.clone());

        transact_write!(
            cli,
            user.create_transact_write_item(),
            user_phone.create_transact_write_item()
        )?;

        user
    } else {
        let user_phone = res.first().cloned().ok_or(Error::Unauthorized(
            "No user linked with the given phone number".into(),
        ))?;
        User::get(cli, &user_phone.pk, Some(EntityType::User))
            .await?
            .ok_or(Error::Unauthorized(
                "No user linked with the given phone number".into(),
            ))?
    };

    Ok(user)
}

pub async fn login_with_oauth(
    cli: &aws_sdk_dynamodb::Client,
    provider: Provider,
    access_token: String,
) -> Result<User> {
    let email = provider.get_email(&access_token).await?;

    let user = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1))
        .await?
        .0
        .get(0)
        .cloned()
        .ok_or(Error::Unauthorized(
            "No user found with the given email".into(),
        ))?;

    Ok(user)
}

pub async fn login_with_email(
    cli: &aws_sdk_dynamodb::Client,
    email: String,
    password: String,
) -> Result<User> {
    let hashed_password = hash_password(&password);
    let (u, _) = User::find_by_email_and_password(
        cli,
        &email,
        UserQueryOption::builder().sk(hashed_password),
    )
    .await?;
    let user = u
        .get(0)
        .cloned()
        .ok_or(Error::Unauthorized("Invalid email or password".into()))?;

    // FIXME(migrate): fallback to tricky migration from postgres
    // let user = if user.is_none() {
    //     migrate_by_email_password(cli, pool, email, password)
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Failed to migrate user by email: {}", e);
    //             Error::Unauthorized("Invalid email or password".into())
    //         })?
    // } else {
    //     user.unwrap()
    // };

    Ok(user)
}

pub async fn login_with_telegram(
    cli: &aws_sdk_dynamodb::Client,
    telegram_raw: String,
) -> Result<User> {
    let telegram_user = parse_telegram_raw(telegram_raw.clone()).map_err(|e| {
        tracing::error!("Failed to parse telegram raw data: {}", e);
        Error::Unauthorized("Invalid telegram data".into())
    })?;
    tracing::debug!("Parsed telegram user: {:?}", telegram_user);
    let (res, _) = UserTelegram::find_by_telegram_id(
        cli,
        telegram_user.id,
        UserTelegramQueryOption::builder().limit(1),
    )
    .await?;
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
        let user = User::new(
            display_name,
            email,
            telegram_user.photo_url.unwrap_or_default(),
            false,
            false,
            crate::types::UserType::Anonymous,
            username,
            None,
        );
        let user_telegram = UserTelegram::new(user.pk.clone(), telegram_user.id, telegram_raw);

        transact_write!(
            cli,
            user.create_transact_write_item(),
            user_telegram.create_transact_write_item()
        )?;
        user
    } else {
        let user_telegram = res.first().cloned().ok_or(Error::Unauthorized(
            "No user linked with the given telegram account".into(),
        ))?;
        User::get(cli, &user_telegram.pk, Some(EntityType::User))
            .await?
            .ok_or(Error::Unauthorized(
                "No user linked with the given telegram account".into(),
            ))?
    };

    Ok(user)
}
