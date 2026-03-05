use crate::*;
use common::services::PointTransactionResponse;

pub type ListPointTransactionsResponse = ListResponse<PointTransactionResponse>;

#[get("/api/me/points/transactions?month&bookmark", user: ratel_auth::User)]
pub async fn list_point_transactions_handler(
    month: Option<String>,
    bookmark: Option<String>,
) -> Result<ListPointTransactionsResponse> {
    let month = month.unwrap_or_else(|| utils::time::current_month());

    let cfg = common::CommonConfig::default();
    let biyard = cfg.biyard();
    let res = biyard
        .list_user_transactions(user.pk.clone(), month, bookmark, Some(10))
        .await?;

    Ok(res)
}
