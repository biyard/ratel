use crate::config;
use crate::sqlx::Pool;
use crate::sqlx::Postgres;
use bdk::prelude::*;
use by_axum::axum::Json;
use dto::Membership;
use dto::User;
use dto::{Result, by_axum::axum::extract::State, reqwest::header::HeaderMap};
use regex::Regex;

pub async fn authorization_noncelab_token(headers: &HeaderMap) -> bool {
    let config = config::get();
    let noncelab_token = config.noncelab_token;

    if let Some(auth_header_value) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header_value.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                return token.trim() == noncelab_token;
            }
        }
    }

    false
}

pub async fn register_users_by_noncelab_handler(
    State(pool): State<Pool<Postgres>>,
    headers: HeaderMap,
    Json(req): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>> {
    tracing::debug!("Registering user with request: {:?}", req);
    let repo = User::get_repository(pool.clone());

    let authorized = authorization_noncelab_token(&headers).await;
    if !authorized {
        return Err(dto::Error::Unauthorized)?;
    }

    let phone_regex = Regex::new(r"^(01[0|1|6|7|8|9]-\d{3,4}-\d{4}|01[0|1|6|7|8|9]\d{8})$")
        .map_err(|_| dto::Error::InvalidPhoneNumberFormat)?;

    if !phone_regex.is_match(&req.phone_number.clone()) {
        return Err(dto::Error::InvalidPhoneNumberFormat)?;
    }

    let user = User::query_builder()
        .nickname_equals(req.display_name.clone())
        .query()
        .map(User::from)
        .fetch_optional(&pool)
        .await?;

    if user.is_some() {
        return Err(dto::Error::UserAlreadyExists)?;
    }

    let user = repo
        .insert(
            req.display_name.clone(),
            req.principal.clone(),
            req.display_name.clone(),
            req.profile_url.unwrap_or_default(),
            true,
            true,
            dto::UserType::Individual,
            None,
            req.username.unwrap_or_default(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            Membership::Free,
            "".to_string(),
            Some(req.phone_number),
            None,
        )
        .await?;

    Ok(Json(RegisterUserResponse {
        user_id: user.id,
        principal: req.principal.clone(),
    }))
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct RegisterUserRequest {
    #[schemars(description = "User's display name shown publicly")]
    pub display_name: String,

    #[schemars(description = "Optional unique username (can be null)")]
    pub username: Option<String>,

    #[schemars(description = "Principal of ICP (Internet Computer Protocol)")]
    pub principal: String,

    #[schemars(description = "unique phone number ex)010-1234-5678 (can not be null)")]
    pub phone_number: String,

    #[schemars(description = "Optional profile url (can be null)")]
    pub profile_url: Option<String>,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct RegisterUserResponse {
    #[schemars(description = "User ID in Ratel")]
    pub user_id: i64,
    #[schemars(description = "Principal of ICP (Internet Computer Protocol)")]
    pub principal: String,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct PathParams {
    pub user_id: i64,
}
