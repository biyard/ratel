use crate::{
    AppState, Error2,
    constants::SESSION_KEY_USER_ID,
    models::user::{
        User, UserEvmAddress, UserOAuth, UserOAuthQueryOption, UserPhoneNumber, UserReferralCode,
    },
    types::{Provider, UserType},
    utils::{
        dynamo_extractor::extract_user_from_session,
        firebase,
        password::hash_password,
        referal_code::generate_referral_code,
        telegram::parse_telegram_raw,
        validator::{validate_image_url, validate_nickname},
    },
};
use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::axum::{
        Extension,
        extract::{Json, State},
    },
};
use serde::Deserialize;
use tower_sessions::Session;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema, Validate)]
pub struct SignupRequest {
    #[serde(flatten)]
    pub signup_type: SignupType,
    #[validate(custom(function = "validate_nickname"))]
    pub display_name: String,
    pub username: String,
    #[validate(custom(function = "validate_image_url"))]
    pub profile_url: String,
    pub description: String,
    pub term_agreed: bool,
    pub informed_agreed: bool,

    pub evm_address: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum SignupType {
    Email {
        #[validate(email)]
        email: String,
        password: String,
    },
    OAuth {
        provider: Provider,
        token: String,
    },
    Telegram {
        telegram_raw: String,
    },
}
///
/// Signup handler
/// Anonymous users can also use this endpoint to convert to normal users.
/// But for
///
pub async fn signup_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Json(req): Json<SignupRequest>,
) -> Result<Json<User>, Error2> {
    tracing::info!("signup_handler: req = {:?}", req);
    req.validate()
        .map_err(|e| Error2::BadRequest(format!("Invalid input: {}", e)))?;

    let anonymous_user = extract_user_from_session(&dynamo.client, &session)
        .await
        .ok();
    let mut payload = UserPayload {
        display_name: req.display_name,
        username: req.username,
        profile_url: req.profile_url,
        description: req.description,
        term_agreed: req.term_agreed,
        informed_agreed: req.informed_agreed,
        email: match &req.signup_type {
            SignupType::Email { email, .. } => email.clone(),
            _ => "".to_string(),
        },
        password: match &req.signup_type {
            SignupType::Email { password, .. } => Some(password.clone()),
            _ => None,
        },
    };
    // Validate Duplicate
    let user = match req.signup_type {
        SignupType::Email { email, password } => {
            let (users, _) =
                User::find_by_email(&dynamo.client, &email, Default::default()).await?;
            if users.len() > 0 {
                return Err(Error2::Duplicate(format!(
                    "Email already registered: {}",
                    email
                )));
            }
            payload.email = email;
            payload.password = Some(hash_password(&password));

            create_or_update_user(&dynamo.client, anonymous_user, payload).await?
        }
        SignupType::OAuth { provider, token } => {
            let uid = match provider {
                Provider::Google => firebase::oauth::verify_token(&token).await?,
            };
            let (user_oauths, _) = UserOAuth::find_by_provider_and_uid(
                &dynamo.client,
                provider.to_string(),
                UserOAuthQueryOption::builder().sk(uid.clone()),
            )
            .await?;
            if user_oauths.len() > 0 {
                return Err(Error2::Duplicate(format!("OAuth already registered")));
            }
            create_or_update_user(&dynamo.client, anonymous_user, payload).await?
        }
        SignupType::Telegram { telegram_raw } => {
            let _telegram_user = parse_telegram_raw(telegram_raw);

            return Err(Error2::InternalServerError("Not implemented yet".into()));
        }
    };
    if let Some(evm_address) = req.evm_address {
        UserEvmAddress::new(user.pk.clone(), evm_address)
            .create(&dynamo.client)
            .await?;
    }
    if let Some(phone_number) = req.phone_number {
        UserPhoneNumber::new(user.pk.clone(), phone_number)
            .create(&dynamo.client)
            .await?;
    }
    UserReferralCode::new(user.pk.clone(), generate_referral_code())
        .create(&dynamo.client)
        .await?;

    session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await?;

    Ok(Json(user))
}

struct UserPayload {
    display_name: String,
    username: String,
    profile_url: String,
    description: String,
    term_agreed: bool,
    informed_agreed: bool,
    email: String,
    password: Option<String>,
}

async fn create_or_update_user(
    dynamo: &aws_sdk_dynamodb::Client,
    user: Option<User>,
    payload: UserPayload,
) -> Result<User, Error2> {
    if let Some(existing_user) = user {
        let mut updater = User::updater(&existing_user.pk, &existing_user.sk)
            .with_display_name(payload.display_name)
            .with_profile_url(payload.profile_url)
            .with_username(payload.username)
            .with_email(payload.email)
            .with_description(payload.description);

        if let Some(password) = payload.password {
            updater = updater.with_password(password);
        } else {
            updater = updater.remove_password();
        };
        updater.execute(dynamo).await?;
        Ok(existing_user)
    } else {
        let user = User::new(
            payload.display_name,
            payload.email,
            payload.profile_url,
            payload.term_agreed,
            payload.informed_agreed,
            UserType::Individual,
            payload.username,
            payload.password,
        );
        user.create(dynamo).await?;
        Ok(user)
    }
}
