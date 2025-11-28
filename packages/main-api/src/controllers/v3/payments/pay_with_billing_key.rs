use std::time::Duration;

use tokio::time::sleep;

use crate::{
    features::{
        membership::*,
        payment::{
            TransactionType, UserPayment,
            user_purchase::{self, UserPurchase},
        },
    },
    types::{CompositePartition, EntityType, Partition},
    *,
};

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct PayWithBillingKeyRequest {
    #[schemars(description = "memberhsip tier to be paid for")]
    pub membership: MembershipTier,
    pub card_number: String,
    pub expiry_year: String,
    pub expiry_month: String,
    pub birth_or_business_registration_number: String,
    pub password_two_digits: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct PayWithBillingKeyResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
    #[schemars(description = "Payment transaction ID")]
    pub transaction_id: String,
    #[schemars(description = "Membership tier purchased")]
    pub membership_tier: String,
    #[schemars(description = "Amount paid in dollars")]
    pub amount: i64,
    #[schemars(description = "Duration of membership in days")]
    pub duration_days: i32,
    #[schemars(description = "Credits included with membership")]
    pub credits: i64,
    #[schemars(description = "Payment timestamp (Unix timestamp in microseconds)")]
    pub paid_at: i64,
}

pub async fn pay_with_billing_key_handler(
    State(AppState {
        dynamo, portone, ..
    }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(PayWithBillingKeyRequest {
        membership,
        card_number,
        expiry_year,
        expiry_month,
        birth_or_business_registration_number,
        password_two_digits,
    }): Json<PayWithBillingKeyRequest>,
) -> Result<Json<PayWithBillingKeyResponse>> {
    let pk = CompositePartition::user_payment_pk(user.pk.clone());
    let mut user_payment: UserPayment = UserPayment::get(&dynamo.client, &pk, None::<String>)
        .await?
        .ok_or_else(|| Error::InvalidIdentification)?;

    let mut txs = vec![];

    if user_payment.billing_key.is_none() {
        let res = portone
            .get_billing_key(
                user_payment.pk.to_string(),
                user_payment.name.clone(),
                card_number,
                expiry_year,
                expiry_month,
                birth_or_business_registration_number,
                password_two_digits,
            )
            .await?;

        let tx = UserPayment::updater(&user.pk, EntityType::UserPayment)
            .with_billing_key(res.billing_key_info.billing_key.clone())
            .transact_write_item();

        txs.push(tx);

        user_payment.billing_key = Some(res.billing_key_info.billing_key.clone());

        debug!("Billing key response: {:?}", res);
    }

    let tx_type = TransactionType::PurchaseMembership(membership.to_string());
    let membership_pk: Partition = membership.into();

    let membership: Membership =
        Membership::get(&dynamo.client, membership_pk, Some(EntityType::Membership))
            .await?
            .ok_or(Error::NotFound("Membership not found".to_string()))?;
    // NOTE: it's for enterprise membership only
    if !membership.is_active {
        return Err(Error::ExpiredMembership);
    }

    let amount = membership.price_dollars;

    let mut user_purchase = UserPurchase::new(user.pk.clone().into(), tx_type, amount);

    let res = portone
        .pay_with_billing_key(
            user_purchase.payment_id.clone(),
            user_payment.customer_id.clone(),
            user_payment.name.clone(),
            user_purchase.tx_type.to_string(),
            user_payment.billing_key.clone().unwrap(),
            amount,
        )
        .await?;

    user_purchase.tx_id = Some(res.payment.pg_tx_id.clone());

    txs.push(user_purchase.create_transact_write_item());
    let tx = UserMembership::new(
        user.pk.clone().into(),
        membership.pk.into(),
        membership.duration_days,
        membership.credits,
    )?
    .with_purchase_id(user_purchase.pk.clone())
    .with_auto_renew(true)
    .upsert_transact_write_item();

    txs.push(tx);
    while let Err(e) = transact_write_items!(&dynamo.client, txs.clone()) {
        error!("Error in transact write items: {:?}", e);
        sleep(Duration::from_millis(100)).await;
    }

    // TODO: Make auto renew cronjob

    return Ok(Json(PayWithBillingKeyResponse {
        status: "Payment successful".to_string(),
        transaction_id: user_purchase.payment_id.clone(),
        membership_tier: membership.tier.to_string(),
        amount: membership.price_dollars,
        duration_days: membership.duration_days,
        credits: membership.credits,
        paid_at: user_purchase.created_at,
    }));
}
