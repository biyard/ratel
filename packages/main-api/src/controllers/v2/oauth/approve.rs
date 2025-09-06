use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use bdk::prelude::*;
use dto::{
    AuthCode, Result,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};
use rand::{TryRngCore, rngs::OsRng};

use crate::{
    Error,
    models::oauth::scope::{Scope, deserialize_scope_vec},
    utils::users::extract_user_id,
};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct ApproveRequest {
    pub client_id: String,
    pub redirect_uri: String,
    #[serde(deserialize_with = "deserialize_scope_vec")]
    pub scope: Vec<Scope>,
    pub state: String,
}
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct Response {
    pub redirect_url: String,
}

pub fn generate_random_string() -> Result<String> {
    let mut key = [0u8; 32];

    OsRng.try_fill_bytes(&mut key).map_err(|e| {
        tracing::error!("Failed to generate random string: {}", e);
        Error::ServerError(e.to_string())
    })?;

    Ok(URL_SAFE_NO_PAD.encode(key))
}

pub async fn approve_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(req): Json<ApproveRequest>,
) -> Result<Json<Response>> {
    let user_id = extract_user_id(&pool, auth).await?;
    let code = generate_random_string()?;
    AuthCode::get_repository(pool)
        .insert(user_id, code.clone(), req.client_id.clone(), 3600)
        .await?;
    tracing::debug!("Request: {:?}", req);
    let url = format!(
        "{}?code={}{}",
        req.redirect_uri,
        code,
        if req.state.is_empty() {
            "".to_string()
        } else {
            format!("&state={}", req.state)
        }
    );
    Ok(Json(Response { redirect_url: url }))
}
