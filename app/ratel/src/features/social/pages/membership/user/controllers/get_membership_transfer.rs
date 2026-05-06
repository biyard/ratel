use super::super::{models::UserPurchaseLocal, *};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PurchaseHistoryItem {
    pub tx_type: String,
    pub amount: i64,
    pub payment_id: String,
    pub tx_id: String,
    pub created_at: i64,
}

impl From<UserPurchaseLocal> for PurchaseHistoryItem {
    fn from(purchase: UserPurchaseLocal) -> Self {
        Self {
            tx_type: purchase.tx_type.to_string(),
            amount: purchase.amount,
            payment_id: purchase.payment_id,
            tx_id: purchase.tx_id,
            created_at: purchase.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PurchaseHistoryResponse {
    pub items: Vec<PurchaseHistoryItem>,
    pub bookmark: Option<String>,
}

#[get("/api/me/membership/history?bookmark", user: crate::features::auth::User)]
pub async fn get_purchase_history_handler(
    bookmark: Option<String>,
) -> Result<PurchaseHistoryResponse> {
    use super::super::models::UserPurchaseLocalQueryOption;

    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let mut opt = UserPurchaseLocalQueryOption::builder()
        .limit(20)
        .scan_index_forward(false);

    if let Some(bookmark_str) = bookmark {
        opt = opt.bookmark(bookmark_str);
    }

    let (purchases, last_key) = UserPurchaseLocal::find_by_user(
        cli,
        CompositePartition::user_purchase_pk(user.pk.clone()),
        opt,
    )
    .await?;

    let items: Vec<PurchaseHistoryItem> = purchases.into_iter().map(Into::into).collect();

    Ok(PurchaseHistoryResponse {
        items,
        bookmark: last_key,
    })
}
