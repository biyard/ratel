// Migrated from packages/main-api/src/controllers/v3/auth/signup.rs
use crate::models::*;
#[cfg(feature = "server")]
use crate::utils::{
    password::hash_password,
    referral_code::generate_referral_code,
    validator::{validate_image_url, validate_username},
};
use crate::*;
#[cfg(feature = "server")]
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(aide::OperationIo, schemars::JsonSchema, Validate)
)]
pub struct SignupRequest {
    #[serde(flatten)]
    pub signup_type: SignupType,
    pub display_name: String,
    #[cfg_attr(feature = "server", validate(custom(function = "validate_username")))]
    pub username: String,
    #[cfg_attr(feature = "server", validate(custom(function = "validate_image_url")))]
    pub profile_url: String,
    pub description: String,
    pub term_agreed: bool,
    pub informed_agreed: bool,

    pub evm_address: Option<String>,
    pub phone_number: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
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
        provider: OauthProvider,
        access_token: String,
    },
    Telegram {
        telegram_raw: String,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SignupResponse {
    #[serde(flatten)]
    pub user: User,
    pub refresh_token: Option<String>,
}

///
/// Signup handler
/// Anonymous users can also use this endpoint to convert to normal users.
/// But for
///
#[post("/api/auth/signup", session: Extension<tower_sessions::Session>)]
pub async fn signup_handler(req: SignupRequest) -> Result<SignupResponse> {
    let cli = crate::config::get().dynamodb();
    let Extension(session) = session;

    tracing::info!("signup_handler: req = {:?}", req);
    req.validate()
        .map_err(|e| Error::BadRequest(format!("Invalid input: {}", e)))?;

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
            unimplemented!()
        }
    };
    if let Some(evm_address) = evm_address {
        UserEvmAddress::new(user.pk.clone(), evm_address)
            .create(cli)
            .await?;
    }

    UserReferralCode::new(user.pk.clone(), generate_referral_code())
        .create(cli)
        .await?;

    session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await?;

    let device_id: Option<String> = req.device_id.clone();

    let refresh_token = if let Some(device_id) = device_id {
        let (rt, plain) = UserRefreshToken::new(&user, device_id);
        rt.upsert(cli).await?;

        Some(plain)
    } else {
        None
    };

    Ok(SignupResponse {
        user,
        refresh_token,
    })
}

#[cfg(feature = "server")]
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
) -> Result<User> {
    tracing::debug!("Signing up with email: {}", email);

    let is_invalid = EmailVerification::find_by_email_and_code(
        cli,
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

    let (users, _) = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1)).await?;
    if users.len() > 0 {
        return Err(Error::Duplicate(format!(
            "Email already registered: {}",
            email
        )));
    }
    let hashed_password = hash_password(&password);

    let user = User::new(
        display_name,
        email,
        profile_url,
        term_agreed,
        informed_agreed,
        UserType::Individual,
        username,
        Some(hashed_password),
    );

    user.create(cli).await?;

    Ok(user)
}

#[cfg(feature = "server")]
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
    provider: OauthProvider,
    access_token: String,
) -> Result<User> {
    tracing::debug!("Verifying id_token with provider: {:?}", provider);
    let email = provider.get_email(&access_token).await?;

    let (user, _bookmark) =
        User::find_by_email(cli, &email, UserQueryOption::builder().limit(1)).await?;
    if user.len() > 0 {
        return Err(Error::Duplicate(format!(
            "Email already registered: {}",
            email
        )));
    }
    let user = User::new(
        display_name,
        email,
        profile_url,
        term_agreed,
        informed_agreed,
        UserType::Individual,
        username,
        None,
    );

    user.create(cli).await?;

    Ok(user)
}

#[cfg(feature = "server")]
async fn signup_with_phone(
    cli: &aws_sdk_dynamodb::Client,
    phone: String,
    code: String,
) -> Result<User> {
    tracing::debug!("Signing up with phone: {}", phone);

    let is_invalid = PhoneVerification::find_by_phone_and_code(
        cli,
        phone.clone(),
        PhoneVerificationQueryOption::builder()
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

    let (users, _) = User::find_by_phone(cli, &phone, UserQueryOption::builder().limit(1)).await?;

    if users.len() > 0 {
        return Ok(users[0].clone());
    }

    let user = User::new_phone(phone);

    user.create(cli).await?;

    Ok(user)
}
