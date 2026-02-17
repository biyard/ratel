use crate::constants::SESSION_KEY_USER_ID;
use crate::models::UserRefreshToken;
use crate::utils::sha256_baseurl::sha256_base64url;
use crate::utils::time::now;
use crate::utils::uuid::sorted_uuid;

use common::models::*;
use common::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChangeAccountRequest {
    pub device_id: String,
    pub user_pk: Partition,
    pub refresh_token: String,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChangeAccountResponse {
    #[serde(flatten)]
    pub user: User,
    pub refresh_token: Option<String>,
}

#[post("/api/auth/change-account", session: Extension<TowerSession>)]
pub async fn change_account_handler(
    form: dioxus::fullstack::Form<ChangeAccountRequest>,
) -> std::result::Result<ChangeAccountResponse, ServerFnError> {
    let c = crate::config::get();
    let cli = c.common.dynamodb();
    let req: ChangeAccountRequest = form.0;

    let target_sk = EntityType::UserRefreshToken(req.device_id.clone());

    let rt: UserRefreshToken = UserRefreshToken::get(cli, req.user_pk.clone(), Some(target_sk))
        .await
        .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?
        .ok_or_else(|| ServerFnError::new("Account not found for this device"))?;

    let now_ts = now();

    if rt.revoked {
        return Err(ServerFnError::new("Token revoked"));
    }
    if let Some(exp) = rt.expired_at {
        if exp < now_ts {
            return Err(ServerFnError::new("Token expired"));
        }
    }

    let req_hash = sha256_base64url(&req.refresh_token);
    if rt.token_hash != req_hash {
        return Err(ServerFnError::new("Invalid refresh token"));
    }

    let new_plain = sorted_uuid();
    let new_hash = sha256_base64url(&new_plain);

    UserRefreshToken::updater(rt.pk.clone(), rt.sk.clone())
        .with_token_hash(new_hash)
        .with_created_at(now_ts)
        .with_revoked(false)
        .execute(cli)
        .await
        .map_err(|e| ServerFnError::new(format!("DB update failed: {:?}", e)))?;

    session
        .insert(SESSION_KEY_USER_ID, rt.pk.to_string())
        .await
        .map_err(|e| ServerFnError::new(format!("Session insert failed: {:?}", e)))?;

    let user: User = User::get(cli, req.user_pk, Some(EntityType::User))
        .await
        .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?
        .unwrap_or_default();

    Ok(ChangeAccountResponse {
        user,
        refresh_token: Some(new_plain),
    })
}
