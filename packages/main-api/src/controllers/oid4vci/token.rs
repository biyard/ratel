use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State},
};
use dto::{JsonSchema, Result, aide, sqlx::PgPool};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::utils::users::extract_user_id;

/// OpenID4VCI Token Request
///
/// Request body for obtaining an access token for credential issuance.
/// Supports multiple grant types as defined in OpenID4VCI specification.
///
/// Reference: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html#name-token-endpoint
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct TokenRequest {
    #[schemars(description = "OAuth 2.0 grant type")]
    pub grant_type: String,

    #[schemars(description = "Client identifier")]
    pub client_id: Option<String>,

    #[schemars(description = "Client secret for authentication")]
    pub client_secret: Option<String>,

    #[schemars(description = "Authorization code (for authorization_code grant)")]
    pub code: Option<String>,

    #[schemars(description = "Code verifier for PKCE")]
    pub code_verifier: Option<String>,

    #[schemars(description = "Redirect URI used in authorization request")]
    pub redirect_uri: Option<String>,

    #[schemars(
        description = "Pre-authorized code (for urn:ietf:params:oauth:grant-type:pre-authorized_code grant)"
    )]
    pub pre_authorized_code: Option<String>,

    #[schemars(description = "User PIN for pre-authorized flow")]
    pub user_pin: Option<String>,

    #[schemars(description = "Requested scope")]
    pub scope: Option<String>,
}

/// OpenID4VCI Token Response
///
/// Response containing the access token and related information
/// for credential issuance requests.
#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct TokenResponse {
    #[schemars(description = "Access token for credential requests")]
    pub access_token: String,

    #[schemars(description = "Token type (always 'Bearer')")]
    pub token_type: String,

    #[schemars(description = "Token expiration time in seconds")]
    pub expires_in: u64,

    #[schemars(description = "Scope of the access token")]
    pub scope: Option<String>,

    #[schemars(description = "C_nonce for proof of possession")]
    pub c_nonce: Option<String>,

    #[schemars(description = "C_nonce expiration time in seconds")]
    pub c_nonce_expires_in: Option<u64>,
}

/// Grant Types supported by the credential issuer
#[derive(Debug, Clone, PartialEq)]
pub enum GrantType {
    /// Standard OAuth 2.0 authorization code flow
    AuthorizationCode,
    /// Pre-authorized code flow (credential offer scenario)
    PreAuthorizedCode,
}

impl std::str::FromStr for GrantType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "authorization_code" => Ok(GrantType::AuthorizationCode),
            "urn:ietf:params:oauth:grant-type:pre-authorized_code" => {
                Ok(GrantType::PreAuthorizedCode)
            }
            _ => Err(format!("Unsupported grant type: {}", s)),
        }
    }
}

impl ToString for GrantType {
    fn to_string(&self) -> String {
        match self {
            GrantType::AuthorizationCode => "authorization_code".to_string(),
            GrantType::PreAuthorizedCode => {
                "urn:ietf:params:oauth:grant-type:pre-authorized_code".to_string()
            }
        }
    }
}

/// OID4VCI Token Endpoint Handler
///
/// Processes token requests and issues access tokens for credential issuance.
/// Supports both authorization_code and pre-authorized_code grant types.
pub async fn oid4vci_token_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(request): Json<TokenRequest>,
) -> Result<Json<TokenResponse>> {
    tracing::debug!("OID4VCI token request: {:?}", request);

    // Parse grant type
    let grant_type = request
        .grant_type
        .parse::<GrantType>()
        .map_err(|e| dto::Error::Unknown(e))?;

    // Generate access token and c_nonce
    let access_token = generate_access_token();
    let c_nonce = generate_c_nonce();

    // Token expires in 1 hour
    let expires_in = 3600;
    // C_nonce expires in 10 minutes
    let c_nonce_expires_in = 600;

    match grant_type {
        GrantType::AuthorizationCode => {
            handle_authorization_code_grant(
                &pool,
                auth,
                request,
                access_token,
                c_nonce,
                expires_in,
                c_nonce_expires_in,
            )
            .await
        }
        GrantType::PreAuthorizedCode => {
            handle_pre_authorized_code_grant(
                &pool,
                auth,
                request,
                access_token,
                c_nonce,
                expires_in,
                c_nonce_expires_in,
            )
            .await
        }
    }
}

/// Handle authorization code grant type
async fn handle_authorization_code_grant(
    pool: &PgPool,
    auth: Option<Authorization>,
    request: TokenRequest,
    access_token: String,
    c_nonce: String,
    expires_in: u64,
    c_nonce_expires_in: u64,
) -> Result<Json<TokenResponse>> {
    // Validate required parameters
    let code = request.code.ok_or_else(|| {
        dto::Error::Unknown("code is required for authorization_code grant".to_string())
    })?;

    let redirect_uri = request.redirect_uri.ok_or_else(|| {
        dto::Error::Unknown("redirect_uri is required for authorization_code grant".to_string())
    })?;

    // Extract user ID for token association
    let _user_id = extract_user_id(pool, auth).await?;

    // TODO: Validate the authorization code against stored codes
    // This would typically involve:
    // 1. Looking up the code in the database
    // 2. Checking if it's expired
    // 3. Verifying the redirect_uri matches
    // 4. Ensuring it hasn't been used before

    tracing::info!(
        "Processing authorization code: {} for redirect_uri: {}",
        code,
        redirect_uri
    );

    // For now, we'll accept any valid-looking code
    if code.len() < 10 {
        return Err(dto::Error::Unknown(
            "Invalid authorization code".to_string(),
        ));
    }

    // TODO: Store the issued token in database for later validation
    // This should include:
    // - access_token
    // - associated user_id
    // - expiration time
    // - c_nonce and its expiration

    let response = TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in,
        scope: request.scope,
        c_nonce: Some(c_nonce),
        c_nonce_expires_in: Some(c_nonce_expires_in),
    };

    Ok(Json(response))
}

/// Handle pre-authorized code grant type
async fn handle_pre_authorized_code_grant(
    pool: &PgPool,
    auth: Option<Authorization>,
    request: TokenRequest,
    access_token: String,
    c_nonce: String,
    expires_in: u64,
    c_nonce_expires_in: u64,
) -> Result<Json<TokenResponse>> {
    // Validate required parameters
    let pre_authorized_code = request.pre_authorized_code.ok_or_else(|| {
        dto::Error::Unknown(
            "pre_authorized_code is required for pre-authorized_code grant".to_string(),
        )
    })?;

    // Extract user ID for token association (if authenticated)
    let _user_id = if auth.is_some() {
        Some(extract_user_id(pool, auth).await?)
    } else {
        None
    };

    // TODO: Validate the pre-authorized code
    // This would typically involve:
    // 1. Looking up the pre-authorized code in the database
    // 2. Checking if it's expired
    // 3. Verifying the user_pin if required
    // 4. Ensuring it hasn't been used before

    tracing::info!("Processing pre-authorized code: {}", pre_authorized_code);

    // Check if user_pin is required and validate it
    if let Some(user_pin) = request.user_pin {
        tracing::debug!("Validating user PIN: {}", user_pin);
        // TODO: Validate PIN against stored value
        if user_pin.len() < 4 {
            return Err(dto::Error::Unknown("Invalid user PIN".to_string()));
        }
    }

    // For now, we'll accept any valid-looking pre-authorized code
    if pre_authorized_code.len() < 10 {
        return Err(dto::Error::Unknown(
            "Invalid pre-authorized code".to_string(),
        ));
    }

    // TODO: Store the issued token in database for later validation

    let response = TokenResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in,
        scope: request.scope,
        c_nonce: Some(c_nonce),
        c_nonce_expires_in: Some(c_nonce_expires_in),
    };

    Ok(Json(response))
}

/// Generate a secure access token
fn generate_access_token() -> String {
    format!("oid4vci_at_{}", Uuid::new_v4().to_string().replace('-', ""))
}

/// Generate a cryptographic nonce for proof of possession
fn generate_c_nonce() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!(
        "{}_{}",
        timestamp,
        Uuid::new_v4().to_string().replace('-', "")
    )
}
