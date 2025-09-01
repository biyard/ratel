use bdk::prelude::*;
use dto::{
    AuthClient, AuthCode, Error, Result,
    by_axum::{
        auth::generate_jwt,
        axum::{
            Json,
            body::Body,
            extract::{Request, State},
        },
    },
    sqlx::PgPool,
};

use crate::models::oauth::{grant_type::GrantType, scope::Scope};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct TokenRequest {
    pub grant_type: GrantType,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    #[serde(default)]
    pub client_secret: String,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
}

pub async fn token_handler(
    State(pool): State<PgPool>,
    request: Request<Body>,
) -> Result<Json<TokenResponse>> {
    let bytes = match by_axum::axum::body::to_bytes(request.into_body(), usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("can't read request body: {}", e);
            return Err(Error::BadRequest);
        }
    };

    let body_str = String::from_utf8_lossy(&bytes);
    tracing::debug!("request body: {}", body_str);

    let req = match serde_urlencoded::from_bytes::<TokenRequest>(&bytes) {
        Ok(form) => {
            tracing::debug!("successfully parsed form data: {:?}", form);
            form
        }
        Err(e) => {
            tracing::error!("can't parse form data: {}", e);
            return Err(Error::BadRequest);
        }
    };

    AuthClient::query_builder()
        .client_id_equals(req.client_id.clone())
        .query()
        .fetch_one(&pool)
        .await?;

    let code = AuthCode::query_builder()
        .code_equals(req.code)
        .client_id_equals(req.client_id)
        .query()
        .map(AuthCode::from)
        .fetch_one(&pool)
        .await?;

    let mut claims = by_types::Claims {
        sub: code.user_id.to_string(),
        ..Default::default()
    };

    let token = generate_jwt(&mut claims).map_err(|e| {
        tracing::error!("failed to generate jwt: {}", e);
        Error::Unauthorized
    })?;

    AuthCode::get_repository(pool).delete(code.id).await?;
    Ok(Json(TokenResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in: 3600,
        refresh_token: None,
        scope: None,
    }))
    // validate client
}
