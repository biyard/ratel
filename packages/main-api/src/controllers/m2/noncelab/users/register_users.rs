use crate::config;
use crate::sqlx::Pool;
use crate::sqlx::Postgres;
use bdk::prelude::*;
use by_axum::axum::Json;
use dto::Membership;
use dto::User;
use dto::{Result, by_axum::axum::extract::State, reqwest::header::HeaderMap};

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
            req.display_name,
            req.principal.clone(),
            req.phone_number,
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

    #[schemars(description = "unique phone number (can not be null)")]
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
