use super::*;
use crate::features::payment::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct PurchaseHistoryItem {
    #[schemars(description = "Transaction type")]
    pub tx_type: TransactionType,
    #[schemars(description = "Amount in dollars")]
    pub amount: i64,
    #[schemars(description = "Payment ID")]
    pub payment_id: String,
    #[schemars(description = "Transaction ID (if available)")]
    pub tx_id: String,
    #[schemars(description = "Purchase timestamp in milliseconds")]
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

pub async fn get_purchase_history_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(Pagination { bookmark }): PaginationQuery,
) -> Result<Json<ListResponse<PurchaseHistoryItem>>> {
    let cli = &dynamo.client;

    // Query user purchases using the gsi1 index
    let opt = if let Some(bookmark_str) = bookmark {
        UserPurchaseQueryOption::builder()
            .limit(20)
            .bookmark(bookmark_str)
            .scan_index_forward(false)
    } else {
        UserPurchaseQueryOption::builder()
            .limit(20)
            .scan_index_forward(false)
    };

    let (purchases, last_key) = UserPurchase::find_by_user(
        cli,
        CompositePartition(user.pk.clone(), Partition::Purchase),
        opt,
    )
    .await?;

    let items: Vec<PurchaseHistoryItem> = purchases.into_iter().map(Into::into).collect();

    Ok(Json(ListResponse {
        items,
        bookmark: last_key,
    }))
}
