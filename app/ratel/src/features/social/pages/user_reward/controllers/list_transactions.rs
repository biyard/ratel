use super::super::*;
use crate::common::services::PointTransactionResponse;

pub type ListTransactionsResponse = ListResponse<PointTransactionResponse>;

#[cfg(feature = "server")]
async fn fetch_transactions(
    user_pk: Partition,
    month: Option<String>,
    bookmark: Option<String>,
) -> Result<ListTransactionsResponse> {
    let month = month.unwrap_or_else(|| utils::time::current_month());

    let cfg = crate::common::CommonConfig::default();
    let biyard = cfg.biyard();
    let res = biyard
        .list_user_transactions(user_pk, month, bookmark, Some(10))
        .await?;

    Ok(res)
}

#[get("/api/users/points/transactions?username&month&bookmark")]
pub async fn list_user_transactions_handler(
    username: String,
    month: Option<String>,
    bookmark: Option<String>,
) -> Result<ListTransactionsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let (users, _) = crate::features::auth::User::find_by_username(
        cli,
        &username,
        crate::features::auth::User::opt()
            .sk("TS#".to_string())
            .limit(1),
    )
    .await?;
    let user = users
        .into_iter()
        .find(|u| u.username == username)
        .ok_or(Error::NotFound(format!("User not found: {}", username)))?;

    fetch_transactions(user.pk.clone(), month, bookmark).await
}
