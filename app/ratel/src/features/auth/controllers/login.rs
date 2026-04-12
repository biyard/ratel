// Migrated from packages/main-api/src/controllers/v3/auth/login.rs
use crate::features::auth::models::*;
#[cfg(feature = "server")]
use crate::features::auth::utils::evm::{build_siwe_message, generate_nonce, recover_address};
use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
        provider: OauthProvider,
        access_token: String,
    },
    Telegram {
        telegram_raw: String,
    },
    Wallet {
        signature: String,
        evm_address: String,
        message: String,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LoginResponse {
    #[serde(flatten)]
    pub user: User,
    pub refresh_token: Option<String>,
}

#[post("/api/auth/login", session: Extension<tower_sessions::Session>)]
pub async fn login_handler(req: LoginRequest) -> Result<LoginResponse> {
    let conf = crate::features::auth::config::get();
    let cli = conf.dynamodb();
    let Extension(session) = session;

    let user = match req.clone() {
        LoginRequest::Phone {
            phone,
            code,
            device_id: _,
        } => login_with_phone(cli, phone, code).await?,
        LoginRequest::Email {
            email,
            password,
            device_id: _,
        } => login_with_email(cli, email, password).await?,
        LoginRequest::OAuth {
            provider,
            access_token,
        } => login_with_oauth(cli, provider, access_token).await?,
        LoginRequest::Telegram { telegram_raw } => login_with_telegram(cli, telegram_raw).await?,
        LoginRequest::Wallet {
            signature,
            evm_address,
            message,
        } => wallet_login_handler(cli, evm_address, signature, message, &session).await?,
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
        rt.upsert(cli).await?;

        Some(plain)
    } else {
        None
    };

    Ok(LoginResponse {
        user,
        refresh_token,
    })
}

#[cfg(feature = "server")]
pub async fn login_with_phone(
    cli: &aws_sdk_dynamodb::Client,
    phone: String,
    code: String,
) -> Result<User> {
    // Verify the phone verification code
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
            UserType::Anonymous,
            username,
            None, // No password
        );

        let user_phone = UserPhoneNumber::new(user.pk.clone(), phone.clone());

        crate::transact_write!(
            cli,
            user.create_transact_write_item(),
            user_phone.create_transact_write_item()
        )?;

        user
    } else {
        let user_phone = res.first().cloned().ok_or(AuthError::PhoneNotFound)?;
        User::get(cli, &user_phone.pk, Some(EntityType::User))
            .await?
            .ok_or(AuthError::UserNotFound)?
    };

    Ok(user)
}

#[cfg(feature = "server")]
pub async fn login_with_oauth(
    cli: &aws_sdk_dynamodb::Client,
    provider: OauthProvider,
    access_token: String,
) -> Result<User> {
    let email = provider.get_email(&access_token).await?;

    let user = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1))
        .await?
        .0
        .get(0)
        .cloned()
        .ok_or(AuthError::UserNotFound)?;

    Ok(user)
}

#[cfg(feature = "server")]
pub async fn login_with_email(
    cli: &aws_sdk_dynamodb::Client,
    email: String,
    password: String,
) -> Result<User> {
    let hashed_password = crate::common::utils::password::hash_password(&password);
    let (u, _) = User::find_by_email_and_password(
        cli,
        &email,
        UserQueryOption::builder().sk(hashed_password),
    )
    .await?;
    let user = u
        .get(0)
        .cloned()
        .ok_or(AuthError::InvalidCredentials)?;

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

#[cfg(feature = "server")]
pub async fn login_with_telegram(
    cli: &aws_sdk_dynamodb::Client,
    telegram_raw: String,
) -> Result<User> {
    let telegram_user =
        crate::features::auth::utils::telegram::parse_telegram_raw(telegram_raw.clone()).map_err(|e| {
            crate::error!("Failed to parse telegram raw data: {e}");
            AuthError::InvalidTelegramData
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
            UserType::Anonymous,
            username,
            None,
        );
        let user_telegram = UserTelegram::new(user.pk.clone(), telegram_user.id, telegram_raw);

        crate::transact_write!(
            cli,
            user.create_transact_write_item(),
            user_telegram.create_transact_write_item()
        )?;
        user
    } else {
        let user_telegram = res.first().cloned().ok_or(AuthError::UserNotFound)?;
        User::get(cli, &user_telegram.pk, Some(EntityType::User))
            .await?
            .ok_or(AuthError::UserNotFound)?
    };

    Ok(user)
}

/// Returns `is_new_user: true` if the address is not registered.

#[cfg(feature = "server")]
pub async fn wallet_login_handler(
    cli: &aws_sdk_dynamodb::Client,
    address: String,
    signature: String,
    message: String,
    session: &tower_sessions::Session,
) -> Result<User> {
    // Verify nonce from session
    let stored_nonce: Option<String> = session
        .get("wallet_nonce")
        .await
        .map_err(|e| {
            crate::error!("session: {e}");
            AuthError::SessionFailed
        })?;

    let stored_nonce =
        stored_nonce.ok_or_else(|| AuthError::NonceNotFound)?;

    if !message.contains(&stored_nonce) {
        return Err(AuthError::NonceMismatch.into());
    }

    // Verify signature
    let recovered_address = recover_address(&message, &signature)?;
    if recovered_address.to_lowercase() != address.to_lowercase() {
        return Err(AuthError::InvalidSignature.into());
    }

    // Clear nonce (one-time use)
    session.remove::<String>("wallet_nonce").await.ok();

    let (evm_records, _) = UserEvmAddress::find_by_evm(
        cli,
        &address.to_lowercase(),
        UserEvmAddressQueryOption::builder().limit(1),
    )
    .await?;

    if let Some(evm_record) = evm_records.first() {
        let user = User::get(cli, &evm_record.pk, Some(EntityType::User))
            .await?
            .ok_or(AuthError::UserNotFound)?;

        return Ok(user);
    }
    Err(AuthError::UserNotFound.into())
}

// ── Wallet address check ────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct WalletCheckRequest {
    pub evm_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct WalletCheckResponse {
    pub exists: bool,
}

#[post("/api/auth/wallet/check")]
pub async fn wallet_check_handler(req: WalletCheckRequest) -> Result<WalletCheckResponse> {
    let cli = crate::features::auth::config::get().dynamodb();
    let (records, _) = UserEvmAddress::find_by_evm(
        cli,
        &req.evm_address.to_lowercase(),
        UserEvmAddressQueryOption::builder().limit(1),
    )
    .await?;

    Ok(WalletCheckResponse {
        exists: !records.is_empty(),
    })
}

// ── Nonce generation ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct WalletNonceResponse {
    pub nonce: String,
    pub message: String,
}

/// Generate a nonce for SIWE verification (used by both login and signup).
#[post("/api/auth/wallet/nonce", session: Extension<tower_sessions::Session>)]
pub async fn wallet_nonce_handler() -> crate::common::Result<WalletNonceResponse> {
    tracing::info!("[wallet-server] nonce_handler called");
    let Extension(session) = session;
    let nonce = generate_nonce();
    let message = build_siwe_message(&nonce);

    session
        .insert("wallet_nonce", nonce.clone())
        .await
        .map_err(|e| {
            crate::error!("session: {e}");
            AuthError::SessionFailed
        })?;

    Ok(WalletNonceResponse { nonce, message })
}
