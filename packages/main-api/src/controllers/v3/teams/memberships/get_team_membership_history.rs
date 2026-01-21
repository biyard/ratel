use crate::{
    features::payment::*,
    models::{team::Team, user::User},
    *,
};
use by_axum::aide::NoApi;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct TeamPurchaseHistoryItem {
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

impl From<TeamPurchase> for TeamPurchaseHistoryItem {
    fn from(purchase: TeamPurchase) -> Self {
        Self {
            tx_type: purchase.tx_type,
            amount: purchase.amount,
            payment_id: purchase.payment_id,
            tx_id: purchase.tx_id,
            created_at: purchase.created_at,
        }
    }
}

pub async fn get_team_purchase_history_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Extension(team): Extension<Team>,
    Query(Pagination { bookmark }): PaginationQuery,
) -> Result<Json<ListResponse<TeamPurchaseHistoryItem>>> {
    let cli = &dynamo.client;

    // Query team purchases using the gsi1 index
    let opt = if let Some(bookmark_str) = bookmark {
        TeamPurchaseQueryOption::builder()
            .limit(20)
            .bookmark(bookmark_str)
            .scan_index_forward(false)
    } else {
        TeamPurchaseQueryOption::builder()
            .limit(20)
            .scan_index_forward(false)
    };

    let (purchases, last_key) = TeamPurchase::find_by_team(
        cli,
        CompositePartition(team.pk.clone(), Partition::Purchase),
        opt,
    )
    .await?;

    let items: Vec<TeamPurchaseHistoryItem> = purchases.into_iter().map(Into::into).collect();

    Ok(Json(ListResponse {
        items,
        bookmark: last_key,
    }))
}
