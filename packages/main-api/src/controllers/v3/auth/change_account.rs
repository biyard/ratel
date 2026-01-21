use crate::models::UserRefreshToken;
use crate::utils::sha256_baseurl::sha256_base64url;
use crate::*;
use bdk::prelude::*;
use by_axum::axum::{
    Extension,
    extract::{Json, State},
};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
pub struct ChangeAccountRequest {
    pub device_id: String,
    pub user_pk: Partition,
    pub refresh_token: String,
}

#[derive(
    Debug, Default, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema,
)]
pub struct ChangeAccountResponse {
    #[serde(flatten)]
    pub user: User,
    pub refresh_token: Option<String>,
}

pub async fn change_account_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Json(req): Json<ChangeAccountRequest>,
) -> Result<Json<ChangeAccountResponse>> {
    let target_sk = EntityType::UserRefreshToken(req.device_id.clone());

    let rt = UserRefreshToken::get(&dynamo.client, &req.user_pk, Some(target_sk))
        .await?
        .ok_or(Error::Unauthorized(
            "Account not found for this device".into(),
        ))?;

    let now_ts = now();

    if rt.revoked {
        return Err(Error::Unauthorized("Token revoked".into()));
    }
    if let Some(exp) = rt.expired_at {
        if exp < now_ts {
            return Err(Error::Unauthorized("Token expired".into()));
        }
    }

    let req_hash = sha256_base64url(&req.refresh_token);
    if rt.token_hash != req_hash {
        return Err(Error::Unauthorized("Invalid refresh token".into()));
    }

    let new_plain = sorted_uuid();
    let new_hash = sha256_base64url(&new_plain);

    UserRefreshToken::updater(&rt.pk, &rt.sk)
        .with_token_hash(new_hash)
        .with_created_at(now_ts)
        .with_revoked(false)
        .execute(&dynamo.client)
        .await?;

    session
        .insert(SESSION_KEY_USER_ID, rt.pk.to_string())
        .await?;

    let user = User::get(&dynamo.client, &req.user_pk, Some(EntityType::User))
        .await?
        .unwrap_or_default();

    Ok(Json(ChangeAccountResponse {
        user,
        refresh_token: Some(new_plain),
    }))
}
