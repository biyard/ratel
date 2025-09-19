use std::{str::FromStr, time::SystemTime};

use crate::axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode, header::AUTHORIZATION},
    middleware::Next,
};
use by_types::{AuthConfig, Claims, TokenScheme};
use http::header::COOKIE;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rest_api::Signature;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

static mut AUTH_CONFIG: Option<AuthConfig> = None;
pub const USER_SESSION_KEY: &str = "user_session";
pub static mut AUTH_TOKEN_KEY: &str = "auth_token";

pub fn set_auth_token_key(key: &'static str) {
    unsafe {
        AUTH_TOKEN_KEY = key;
    }
}

pub fn set_auth_config(secret: AuthConfig) {
    unsafe {
        AUTH_CONFIG = Some(secret);
    }
}

pub fn get_auth_config() -> &'static AuthConfig {
    #[allow(static_mut_refs)]
    unsafe {
        AUTH_CONFIG
            .as_ref()
            .expect("you must call `set_auth_config` when starting a server")
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Authorization enum
///
/// # Variants
///
/// * `UserSig(Signature)` - User signature
/// * `Bearer(String)` - Bearer token
/// * `Basic` - Basic authentication
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Authorization {
    UserSig(Signature),
    Bearer { claims: Claims },
    Basic { username: String, password: String },
    Session(UserSession),
    ServerKey,
    SecretApiKey,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct UserSession {
    pub user_id: i64,
    pub principal: String,
    pub email: String,
}

/// Authorization middleware
///
/// # Arguments
///
/// * `req` - The request
/// * `next` - The next middleware
///
/// # Returns
///
/// * `Response<Body>` - The response
/// * `StatusCode` - The status code of the response
///
pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization middleware {:?}", req.uri());
    let session = req.extensions().get::<Session>().cloned();

    let auth = if let Some(ref session) = session {
        if let Ok(Some(user_session)) = session.get::<UserSession>(USER_SESSION_KEY).await {
            tracing::debug!("User session found: {:?}", user_session);
            Some(Authorization::Session(user_session))
        } else {
            None
        }
    } else {
        None
    };

    // Process the token if available
    let auth = if let Some(auth) = auth {
        Some(auth)
    } else if let Some((scheme, value)) = extract_auth_token(&req) {
        if let Ok(token_scheme) = TokenScheme::try_from(scheme) {
            tracing::debug!("Token scheme: {:?}", token_scheme);

            let ext = match token_scheme {
                TokenScheme::Bearer => verify_jwt(Some(value)).ok(),
                TokenScheme::Usersig => verify_usersig(Some(value)).ok(),
                TokenScheme::Secret => {
                    if option_env!("ENV").unwrap_or("local") == "prod" {
                        None
                    } else {
                        verify_secret(Some(value)).ok()
                    }
                }
                TokenScheme::XServerKey => verify_server_key(Some(value)).ok(),
                TokenScheme::Session => None,
            };

            ext
        } else {
            None
        }
    } else {
        None
    };
    req.extensions_mut().insert(auth);

    let mut res = next.run(req).await;
    if let Some(session) = session {
        res.extensions_mut().insert(session);
    }

    return Ok(res);
}
/// Extracts authentication token from request headers
/// Prioritizes Authorization header over Cookie header
fn extract_auth_token(req: &Request) -> Option<(&str, &str)> {
    // Try Authorization header first
    if let Some(token) = extract_from_auth_header(req) {
        return Some(token);
    }

    // Fall back to Cookie header
    extract_from_cookie_header(req)
}

fn extract_from_auth_header(req: &Request) -> Option<(&str, &str)> {
    req.headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|auth_value| {
            let mut parts = auth_value.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some(scheme), Some(value)) => Some((scheme, value)),
                _ => None,
            }
        })
}

fn extract_from_cookie_header(req: &Request) -> Option<(&str, &str)> {
    req.headers()
        .get(COOKIE)
        .and_then(|header| header.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str
                .split(';')
                .map(|s| s.trim())
                .find_map(|cookie_pair| {
                    cookie_pair.split_once('=').and_then(|(name, value)| {
                        if name.trim() == unsafe { AUTH_TOKEN_KEY } {
                            let value = value.trim();
                            value.split_once(' ').map(|(scheme, token)| (scheme, token))
                        } else {
                            None
                        }
                    })
                })
        })
}

pub fn verify_server_key(value: Option<&str>) -> Result<Authorization, StatusCode> {
    if value.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let server_key = value.unwrap();
    let server_server_key = option_env!("SERVER_KEY")
        .expect("You must set SERVER_KEY to enable `server key` authentication");

    if server_key == server_server_key {
        return Ok(Authorization::ServerKey);
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub fn verify_secret(value: Option<&str>) -> Result<Authorization, StatusCode> {
    if value.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let api_key = value.unwrap();
    let secret_server_key = option_env!("AUTH_SECRET_KEY")
        .expect("You must set AUTH_SECRET_KEY to enable `secret` authentication");

    if api_key == secret_server_key {
        return Ok(Authorization::SecretApiKey);
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub fn generate_jwt(claims: &mut Claims) -> Result<String, StatusCode> {
    match get_auth_config() {
        &AuthConfig::Jwt {
            ref expiration,
            ref secret,
        } => {
            let exp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| {
                    tracing::error!("Failed to get current time: {}", e);
                    StatusCode::UNAUTHORIZED
                })?
                .as_secs()
                + expiration;
            claims.exp = exp;
            let token = match jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                claims,
                &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
            ) {
                Ok(token) => token,
                Err(err) => {
                    tracing::error!("Error creating JWT: {}", err);
                    return Err(StatusCode::UNAUTHORIZED);
                }
            };

            Ok(token)
        }
    }
}

pub fn verify_jwt(value: Option<&str>) -> Result<Authorization, StatusCode> {
    match get_auth_config() {
        AuthConfig::Jwt { secret, .. } => {
            if let Some(token) = value {
                let claims: Claims = jsonwebtoken::decode(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::new(Algorithm::HS256),
                )
                .map_err(|e| {
                    tracing::error!("Failed to decode token: {}", e);
                    StatusCode::UNAUTHORIZED
                })?
                .claims;

                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|e| {
                        tracing::error!("Failed to get current time: {}", e);
                        StatusCode::UNAUTHORIZED
                    })?
                    .as_secs();

                if now > claims.exp {
                    return Err(StatusCode::UNAUTHORIZED);
                }

                return Ok(Authorization::Bearer { claims });
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
    }
}

/// Verify the user signature
/// You must set the `DOMAIN` environment.
/// The signature is valid for 1 hour.
/// The signature is in the format of `timestamp:signature` for authorization header.
/// The signature is verified using the message `domain-timestamp` and the public key of the user.
///
/// # Arguments
///
/// * `value` - The value of the Authorization header
///
/// # Returns
///
/// * `Signature` - The signature of the user
/// * `StatusCode` - The status code of the response
pub fn verify_usersig(value: Option<&str>) -> Result<Authorization, StatusCode> {
    tracing::debug!("verify_usersig: {:?}", value);
    if let Some((timestamp, signature)) = value.unwrap_or_default().split_once(":") {
        let parsed_timestamp: i64 = timestamp.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;
        if now() - parsed_timestamp >= 3600 {
            tracing::error!("Expired timestamp: {}", timestamp);
            return Err(StatusCode::UNAUTHORIZED);
        }

        let msg = format!(
            "{}-{}",
            option_env!("AUTH_DOMAIN").expect("You must set AUTH_DOMAIN environment"),
            timestamp
        );
        let sig = rest_api::Signature::from_str(signature).map_err(|e| {
            tracing::error!("Failed to parse signature: {}", e);
            StatusCode::UNAUTHORIZED
        })?;
        tracing::debug!("SignMessage: {}", msg);
        let address = sig.verify(&msg).map_err(|e| {
            tracing::error!("Failed to verify signature: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        if address.is_empty() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        return Ok(Authorization::UserSig(sig));
    }

    Err(StatusCode::UNAUTHORIZED)
}
