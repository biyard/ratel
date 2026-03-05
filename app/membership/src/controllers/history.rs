use crate::models::{TransactionType, UserPurchase};
use crate::*;
use ratel_auth::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PurchaseHistoryItem {
    pub tx_type: TransactionType,
    pub amount: i64,
    pub payment_id: String,
    pub tx_id: String,
    pub created_at: i64,
}

impl From<UserPurchase> for PurchaseHistoryItem {
    fn from(purchase: UserPurchase) -> Self {
        Self {
            tx_type: purchase.tx_type,
            amount: purchase.amount,
            payment_id: purchase.payment_id,
            tx_id: purchase.tx_id,
            created_at: purchase.created_at,
        }
    }
}

#[get("/v3/me/memberships/history?bookmark", user: User)]
pub async fn get_purchase_history_handler(
    bookmark: Option<String>,
) -> Result<ListResponse<PurchaseHistoryItem>> {
    use crate::models::UserPurchaseQueryOption;

    let conf = crate::config::get();
    let cli = conf.common.dynamodb();

    let mut opt = UserPurchaseQueryOption::builder()
        .limit(20)
        .scan_index_forward(false);

    if let Some(bookmark_str) = bookmark {
        opt = opt.bookmark(bookmark_str);
    }

    let (purchases, last_key) = UserPurchase::find_by_user(
        cli,
        CompositePartition::user_purchase_pk(user.pk.clone()),
        opt,
    )
    .await?;

    let items: Vec<PurchaseHistoryItem> = purchases.into_iter().map(Into::into).collect();

    Ok(ListResponse {
        items,
        bookmark: last_key,
    })
}
