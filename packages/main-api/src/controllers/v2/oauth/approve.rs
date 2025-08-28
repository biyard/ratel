use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use bdk::prelude::*;
use dto::{
    AuthCode, Result,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::State,
            response::{IntoResponse, Redirect},
        },
    },
    sqlx::PgPool,
};
use rand::RngCore;

use crate::utils::users::extract_user_id;

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
pub struct ApproveRequest {
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: String,
}

fn generate_random_string() -> String {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    URL_SAFE_NO_PAD.encode(key)
}

pub async fn approve_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Json(req): Json<ApproveRequest>,
) -> Result<impl IntoResponse> {
    let user_id = extract_user_id(&pool, auth).await?;
    let code = generate_random_string();
    AuthCode::get_repository(pool)
        .insert(code.clone(), req.client_id.clone(), user_id, 3600)
        .await?;
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
    let url = serde_urlencoded::from_str(&url)?;
    Ok(Redirect::to(url))
}
