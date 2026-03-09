// Migrated from packages/main-api/src/controllers/v3/auth/list_accounts.rs
use crate::features::auth::models::*;
use crate::features::auth::*;
#[cfg(feature = "server")]
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct ListAccountsQueryParams {
    pub device_id: String,
    pub bookmark: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AccountItem {
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub user_type: UserType,
    pub last_login_at: i64,
    pub revoked: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ListAccountsResponse {
    pub items: Vec<AccountItem>,
    pub bookmark: Option<String>,
}

#[get("/api/auth/accounts")]
pub async fn list_accounts_handler(params: ListAccountsQueryParams) -> Result<ListAccountsResponse> {
    let cli = crate::features::auth::config::get().dynamodb();

    let mut query_options = UserRefreshTokenQueryOption::builder().limit(10);

    if let Some(bookmark) = params.bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (rts, next_bookmark) =
        UserRefreshToken::find_by_device_id(cli, &params.device_id, query_options).await?;

    if rts.is_empty() {
        return Ok(ListAccountsResponse {
            items: vec![],
            bookmark: next_bookmark,
        });
    }

    let now_ts = crate::common::utils::time::now();
    let mut seen = HashSet::<String>::new();
    let mut accounts = Vec::<AccountItem>::new();

    for rt in rts.into_iter() {
        if rt.revoked {
            continue;
        }
        if let Some(exp) = rt.expired_at {
            if exp < now_ts {
                continue;
            }
        }

        let k = rt.pk.to_string();
        if !seen.insert(k) {
            continue;
        }

        accounts.push(AccountItem {
            user_pk: rt.pk,
            display_name: rt.user_display_name,
            profile_url: rt.user_profile_url,
            username: rt.user_username,
            user_type: rt.user_type,
            last_login_at: rt.created_at,
            revoked: rt.revoked,
        });
    }

    Ok(ListAccountsResponse {
        items: accounts,
        bookmark: next_bookmark,
    })
}
