use aws_sdk_dynamodb::{Client as DynamoClient, types::AttributeValue};
use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State},
};
use dto::{JsonSchema, Result, aide};
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_item, to_item};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use crate::config;
use crate::models::dynamo_tables::main::vc::{
    credential_offer::CredentialOffer as DbCredentialOffer, oauth_access_token::OAuthAccessToken,
    oauth_authorization_code::OAuthAuthorizationCode,
};
use crate::types::OAuth2Scope;
use crate::utils::users_dynamo::extract_user_id_dynamo;

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
    pub scope: Option<OAuth2Scope>,
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
    pub scope: Option<OAuth2Scope>,

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
    State(dynamo_client): State<Arc<DynamoClient>>,
    Json(request): Json<TokenRequest>,
) -> Result<Json<TokenResponse>> {
    tracing::debug!("OID4VCI token request: {:?}", request);

    let conf = config::get();
    let table_name = &conf.dual_write.table_name;

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
                &dynamo_client,
                &table_name,
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
                &dynamo_client,
                &table_name,
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
    dynamo_client: &DynamoClient,
    table_name: &str,
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
    let _user_id = extract_user_id_dynamo(dynamo_client, table_name, auth).await?;

    // Validate the authorization code against stored codes
    let auth_code =
        validate_authorization_code(dynamo_client, table_name, &code, &redirect_uri).await?;

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

    // Store the issued token in DynamoDB
    let scope_string = request
        .scope
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "credential_issuer".to_string());
    let access_token_record = OAuthAccessToken::new(
        access_token.clone(),
        auth_code.user_id,
        auth_code.client_id,
        scope_string,
        "Bearer".to_string(),
        expires_in as i64,
        c_nonce_expires_in as i64,
    );

    let token_item = to_item(&access_token_record)
        .map_err(|e| dto::Error::Unknown(format!("Failed to serialize access token: {}", e)))?;

    dynamo_client
        .put_item()
        .table_name(table_name)
        .set_item(Some(token_item))
        .send()
        .await
        .map_err(|e| dto::Error::Unknown(format!("DynamoDB put_item failed: {}", e)))?;

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
    dynamo_client: &DynamoClient,
    table_name: &str,
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
    let user_id = if auth.is_some() {
        Some(extract_user_id_dynamo(dynamo_client, table_name, auth).await?)
    } else {
        None
    };

    // Validate the pre-authorized code against stored credential offers
    let credential_offer =
        validate_pre_authorized_code(dynamo_client, table_name, &pre_authorized_code).await?;

    tracing::info!("Processing pre-authorized code: {}", pre_authorized_code);

    // Check if user_pin is required and validate it
    if let Some(user_pin) = request.user_pin {
        tracing::debug!("Validating user PIN: {}", user_pin);
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

    // Store the issued token in DynamoDB
    let scope_string = request
        .scope
        .as_ref()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "credential_issuer".to_string());
    let access_token_record = OAuthAccessToken::new(
        access_token.clone(),
        user_id.unwrap_or_else(|| credential_offer.user_id.clone()),
        "oid4vci-client".to_string(),
        scope_string,
        "Bearer".to_string(),
        expires_in as i64,
        c_nonce_expires_in as i64,
    );

    let token_item = to_item(&access_token_record)
        .map_err(|e| dto::Error::Unknown(format!("Failed to serialize access token: {}", e)))?;

    dynamo_client
        .put_item()
        .table_name(table_name)
        .set_item(Some(token_item))
        .send()
        .await
        .map_err(|e| dto::Error::Unknown(format!("DynamoDB put_item failed: {}", e)))?;

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

/// Validate authorization code against stored codes in DynamoDB
async fn validate_authorization_code(
    dynamo_client: &DynamoClient,
    table_name: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<OAuthAuthorizationCode> {
    let pk_value = format!("OAUTH_TOKEN#{}", code);
    let sk_value = "OAUTH_AUTHORIZATION_CODE".to_string();

    let mut key = HashMap::new();
    key.insert("pk".to_string(), AttributeValue::S(pk_value));
    key.insert("sk".to_string(), AttributeValue::S(sk_value));

    let resp = dynamo_client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key))
        .send()
        .await
        .map_err(|e| dto::Error::Unknown(format!("DynamoDB get_item failed: {}", e)))?;

    match resp.item {
        Some(item) => {
            let auth_code: OAuthAuthorizationCode = from_item(item).map_err(|e| {
                dto::Error::Unknown(format!("Failed to deserialize authorization code: {}", e))
            })?;

            // Validate redirect URI matches
            if auth_code.redirect_uri != redirect_uri {
                return Err(dto::Error::Unknown("Invalid redirect_uri".to_string()));
            }

            // Check if expired
            let now = chrono::Utc::now().timestamp_micros();
            if now > auth_code.expires_at {
                return Err(dto::Error::Unknown(
                    "Authorization code expired".to_string(),
                ));
            }

            // Check if already used
            if auth_code.used {
                return Err(dto::Error::Unknown(
                    "Authorization code already used".to_string(),
                ));
            }

            Ok(auth_code)
        }
        None => Err(dto::Error::Unknown(
            "Authorization code not found".to_string(),
        )),
    }
}

/// Validate pre-authorized code against stored credential offers in DynamoDB
async fn validate_pre_authorized_code(
    dynamo_client: &DynamoClient,
    table_name: &str,
    pre_authorized_code: &str,
) -> Result<DbCredentialOffer> {
    // Search for credential offer with matching pre-authorized code
    let pk_value = format!("NONCE#{}", pre_authorized_code);

    let mut expression_values = HashMap::new();
    expression_values.insert(":pk".to_string(), AttributeValue::S(pk_value));

    let resp = dynamo_client
        .query()
        .table_name(table_name)
        .index_name("gsi2")
        .key_condition_expression("gsi2pk = :pk")
        .set_expression_attribute_values(Some(expression_values))
        .send()
        .await
        .map_err(|e| dto::Error::Unknown(format!("DynamoDB query failed: {}", e)))?;

    if let Some(items) = resp.items {
        if let Some(item) = items.first() {
            let credential_offer: DbCredentialOffer = from_item(item.clone()).map_err(|e| {
                dto::Error::Unknown(format!("Failed to deserialize credential offer: {}", e))
            })?;

            // Check if expired
            let now = chrono::Utc::now().timestamp_micros();
            if now > credential_offer.expires_at {
                return Err(dto::Error::Unknown(
                    "Pre-authorized code expired".to_string(),
                ));
            }

            // Check if already used
            if credential_offer.used {
                return Err(dto::Error::Unknown(
                    "Pre-authorized code already used".to_string(),
                ));
            }

            return Ok(credential_offer);
        }
    }

    Err(dto::Error::Unknown(
        "Pre-authorized code not found".to_string(),
    ))
}
