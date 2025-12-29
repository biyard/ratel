use crate::models::{UserRefreshToken, UserRefreshTokenQueryOption};
use crate::*;
use bdk::prelude::*;
use by_axum::axum::extract::{Json, Query, State};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Deserialize, Serialize, aide::OperationIo, JsonSchema)]
pub struct ListAccountsQueryParams {
    pub device_id: String,
    pub bookmark: Option<String>,
}

#[derive(
    Debug, Default, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema,
)]
pub struct AccountItem {
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub user_type: UserType,
    pub last_login_at: i64,
    pub revoked: bool,
}

#[derive(
    Debug, Default, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema,
)]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountItem>,
    pub bookmark: Option<String>,
}

pub async fn list_accounts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Query(ListAccountsQueryParams {
        device_id,
        bookmark,
    }): Query<ListAccountsQueryParams>,
) -> Result<Json<ListItemsResponse<AccountItem>>> {
    let mut query_options = UserRefreshTokenQueryOption::builder().limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (rts, next_bookmark) =
        UserRefreshToken::find_by_device_id(&dynamo.client, &device_id, query_options).await?;

    if rts.is_empty() {
        return Ok(Json(ListItemsResponse {
            items: vec![],
            bookmark: next_bookmark,
        }));
    }

    let now_ts = now();
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

    Ok(Json(ListItemsResponse {
        items: accounts,
        bookmark: next_bookmark,
    }))
}
