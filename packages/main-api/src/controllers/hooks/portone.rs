use crate::features::payment::UserPurchase;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct PortoneRequest {
    pub payment_id: String,
    pub status: String,
    pub tx_id: String,
}

pub async fn portone_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(headers): NoApi<HeaderMap>,
    Json(req): Json<PortoneRequest>,
) -> Result<()> {
    debug!(
        "Incomming PortOne hook: {:?} with headers {:?}",
        req, headers
    );
    let cli = &dynamo.client;
    let opt = UserPurchase::opt_one();

    let (user_purchase, _bm) = UserPurchase::find_by_payment_id(cli, &req.payment_id, opt).await?;

    let user_purchase = user_purchase
        .first()
        .ok_or_else(|| Error::NoUserPurchaseFound)?;

    let user_id: UserPartition = user_purchase.pk.0.clone().into();
    let amount = user_purchase.amount;
    let payment_id = req.payment_id;

    notify!(
        r#"Membership Renewal\n
  User ID: {user_id}\n
  Amount: {amount}\n
  Payment ID: {payment_id}"#
    );

    Ok(())
}
