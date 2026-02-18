// Migrated from packages/main-api/src/controllers/v3/auth/change_account.rs
use crate::models::*;
use crate::*;

#[cfg(feature = "server")]
use crate::utils::sha256_baseurl::sha256_base64url;
#[cfg(feature = "server")]
use crate::utils::uuid::sorted_uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct ChangeAccountRequest {
    pub device_id: String,
    pub user_pk: Partition,
    pub refresh_token: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ChangeAccountResponse {
    #[serde(flatten)]
    pub user: User,
    pub refresh_token: Option<String>,
}

#[post("/api/auth/change-account", session: Extension<tower_sessions::Session>)]
pub async fn change_account_handler(req: ChangeAccountRequest) -> Result<ChangeAccountResponse> {
    let cli = crate::config::get().dynamodb();
    let Extension(session) = session;

    let target_sk = EntityType::UserRefreshToken(req.device_id.clone());

    let rt = UserRefreshToken::get(cli, &req.user_pk, Some(target_sk))
        .await?
        .ok_or(Error::Unauthorized(
            "Account not found for this device".into(),
        ))?;

    let now_ts = common::utils::time::now();

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
        .execute(cli)
        .await?;

    session
        .insert(SESSION_KEY_USER_ID, rt.pk.to_string())
        .await?;

    let user = User::get(cli, &req.user_pk, Some(EntityType::User))
        .await?
        .unwrap_or_default();

    Ok(ChangeAccountResponse {
        user,
        refresh_token: Some(new_plain),
    })
}
