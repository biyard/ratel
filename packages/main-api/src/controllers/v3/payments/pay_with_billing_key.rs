use crate::{
    features::{
        membership::*,
        payment::{TransactionType, UserPayment, user_purchase::UserPurchase},
    },
    types::EntityType,
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
    let mut user_payment: UserPayment =
        UserPayment::get(&dynamo.client, &user.pk, Some(EntityType::UserPayment))
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

    let amount = match membership {
        MembershipTier::Free => 0,
        MembershipTier::Max => 20,
        MembershipTier::Pro => 50,
        MembershipTier::Vip => 100,
        MembershipTier::Enterprise(ref _s) => {
            // TODO: implement customizable pricing
            1000
        }
    };

    let mut user_purchase = UserPurchase::new(
        user.pk,
        TransactionType::PurchaseMembership(membership.to_string()),
        amount,
    );

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
    // TODO: Update user's membership status upon successful payment

    info!("payment response: {:?}", res);

    return Ok(Json(PayWithBillingKeyResponse {
        status: "Payment successful".to_string(),
    }));
}
