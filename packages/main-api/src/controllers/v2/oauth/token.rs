use bdk::prelude::*;
use dto::{
    AuthClient, AuthCode, Error, Result,
    by_axum::{
        auth::generate_jwt,
        axum::{Json, extract::State},
    },
    sqlx::PgPool,
};

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
pub struct TokenRequest {
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
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
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

pub async fn oauth_token(
    State(pool): State<PgPool>,
    Json(req): Json<TokenRequest>,
) -> Result<Json<TokenResponse>> {
    tracing::debug!("received token request");
    match req.grant_type.as_str() {
        "authorization_code" => {
            tracing::debug!("handling authorization_code grant type");
            // Handle authorization_code grant type
        }
        _ => {
            tracing::error!("unsupported grant type: {}", req.grant_type);
            return Err(Error::BadRequest);
        }
    }

    AuthClient::query_builder()
        .client_id_equals(req.client_id.clone())
        .client_secret_equals(req.client_secret)
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
