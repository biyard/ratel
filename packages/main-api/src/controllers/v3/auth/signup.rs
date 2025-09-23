use crate::{
    AppState, Error2,
    models::user::{
        User, UserEvmAddress, UserPhoneNumber, UserPrincipal, UserReferralCode, UserTelegram,
    },
    types::{Theme, UserType},
    utils::{
        dynamo_extractor::{extract_user, get_principal_from_auth},
        password::hash_password,
        referal_code::generate_referral_code,
        telegram::parse_telegram_raw,
        validator::validate_nickname,
    },
};
use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::{
        auth::{Authorization, DYNAMO_USER_SESSION_KEY, DynamoUserSession},
        axum::{
            Extension,
            extract::{Json, State},
        },
    },
};
use serde::Deserialize;
use tower_sessions::Session;
use uuid::Uuid;
use validator::Validate;

/*
FIXME:
There is a problem with the validation logic for emails. For example, it's possible to sign up with an arbitrary email via an API call, regardless of whether the email has been verified.

1. Email/Password signup -> Receive Email Verification ID and verify `EmailVerification Entry` Email
2. Google OAuth signup -> Extract Email from Google Token
3. Telegram OAuth signup -> Use randomly generated Email (e.g. tg_<telegram_id>@noemail.local)

 */
#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignupRequest {
    pub email: String,
    #[validate(custom(function = "validate_nickname"))]
    pub display_name: String,
    pub username: String,
    pub profile_url: String,
    pub content: String,
    pub term_agreed: bool,
    pub informed_agreed: bool,
    pub theme: Option<Theme>,
    pub password: Option<String>,

    // pub principal: Option<String>,
    pub evm_address: Option<String>,
    pub phone_number: Option<String>,
    pub telegram_raw: Option<String>,
}

///
/// Signup handler
/// Anonymous users can also use this endpoint to convert to normal users.
/// But for
///
pub async fn signup_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Extension(session): Extension<Session>,
    Json(req): Json<SignupRequest>,
) -> Result<(), Error2> {
    //This handler is for existing Principal information only. It does not support any specific actions with Principal.    //
    let principal = get_principal_from_auth(auth.clone()).ok();
    let user = extract_user(&dynamo.client, auth).await;

    let user_pk;
    let user_type;

    // check email is duplicate
    let (users, bookmarks) =
        User::find_by_email(&dynamo.client, &req.email, Default::default()).await?;
    if users.len() > 0 || bookmarks.is_some() {
        return Err(Error2::Duplicate(format!(
            "Email already registered: {}",
            req.email
        )));
    }
    let password = req.password.unwrap_or(Uuid::new_v4().to_string());
    let hashed_password = hash_password(&password)
        .map_err(|e| Error2::InternalServerError(format!("Password hashing error: {}", e)))?;

    // When User is Anonymous, convert to normal user
    match user {
        Ok(u) if u.user_type == UserType::Anonymous => {
            // Update Anonymous user to normal user
            User::updater(u.pk.clone(), u.sk.clone())
                .with_display_name(req.display_name)
                .with_email(req.email)
                .with_profile_url(req.profile_url)
                .with_term_agreed(req.term_agreed)
                .with_informed_agreed(req.informed_agreed)
                .with_user_type(UserType::Individual)
                .with_theme(req.theme.unwrap_or_default())
                .with_password(hashed_password)
                .execute(&dynamo.client)
                .await?;
            user_pk = u.pk;
            user_type = UserType::Individual;
        }
        _ => {
            // Create new User

            let user = User::new(
                req.display_name,
                req.email,
                req.profile_url,
                req.term_agreed,
                req.informed_agreed,
                UserType::Individual,
                None,
                req.username,
                hashed_password,
            );
            user.create(&dynamo.client).await?;
            user_pk = user.pk;
            user_type = user.user_type;
        }
    }

    if let Some(evm_address) = req.evm_address {
        UserEvmAddress::new(user_pk.clone(), evm_address)
            .create(&dynamo.client)
            .await?;
    }
    if let Some(phone_number) = req.phone_number {
        UserPhoneNumber::new(user_pk.clone(), phone_number)
            .create(&dynamo.client)
            .await?;
    }
    if let Some(telegram_raw) = req.telegram_raw {
        let telegram_user = parse_telegram_raw(telegram_raw.clone())?;
        UserTelegram::new(user_pk.clone(), telegram_user.id, telegram_raw)
            .create(&dynamo.client)
            .await?;
    }
    if let Some(principal) = principal {
        let user_principal = UserPrincipal::new(user_pk.clone(), principal);
        user_principal.create(&dynamo.client).await?;
    }

    UserReferralCode::new(user_pk.clone(), generate_referral_code())
        .create(&dynamo.client)
        .await?;

    session
        .insert(
            DYNAMO_USER_SESSION_KEY,
            DynamoUserSession {
                pk: user_pk.to_string(),
                typ: user_type as i64,
            },
        )
        .await?;

    Ok(())
}
