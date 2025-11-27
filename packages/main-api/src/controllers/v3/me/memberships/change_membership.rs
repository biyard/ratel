use crate::features::membership::*;
use crate::features::payment::*;
use crate::services::portone::{Currency, PortOne};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ChangeMembershipRequest {
    #[schemars(description = "memberhsip tier to be paid for")]
    pub membership: MembershipTier,
    pub currency: Currency,
    pub card_info: Option<CardInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct ChangeMembershipResponse {
    #[schemars(description = "Status of the operation")]
    #[serde(default)]
    pub membership: MembershipTier,
    #[serde(default)]
    pub renewal_date: i64,
}

pub async fn change_membership_handler(
    State(AppState {
        dynamo, portone, ..
    }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<ChangeMembershipRequest>,
) -> Result<Json<ChangeMembershipResponse>> {
    tracing::debug!("change_membership_handler request: {:?}", req);
    let cli = &dynamo.client;

    // Try to get existing membership, or create a Free one if it doesn't exist
    let (user_membership, current_membership) = match user.get_membership(cli).await {
        Ok(result) => result,
        Err(Error::NoUserMembershipFound) | Err(Error::NoMembershipFound) => {
            // User doesn't have a membership yet, create a Free membership
            let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
            let free_membership = Membership::get(
                cli,
                free_membership_pk.clone(),
                Some(EntityType::Membership),
            )
            .await?
            .ok_or_else(|| Error::NoMembershipFound)?;

            let user_membership = UserMembership::new(
                user.pk.clone().into(),
                free_membership.pk.clone().into(),
                free_membership.duration_days,
                free_membership.credits,
            )?;
            user_membership.create(cli).await?;

            (user_membership, free_membership)
        }
        Err(e) => return Err(e),
    };

    if current_membership.tier == req.membership {
        return Err(Error::MembershipAlreadyActive);
    }

    let renewal_date = if req.membership < current_membership.tier {
        handle_downgrade_membership(cli, &user_membership, req.membership.clone()).await?;
        user_membership.expired_at + 1
    } else {
        handle_upgrade_membership(
            cli,
            &portone,
            &user_membership,
            current_membership,
            req.membership.clone(),
            req.card_info,
            req.currency,
        )
        .await?;
        now()
    };

    Ok(Json(ChangeMembershipResponse {
        membership: req.membership,
        renewal_date,
    }))
}

/// Handle membership downgrade by scheduling it for next renewal
/// The downgrade takes effect when current membership expires
async fn handle_downgrade_membership(
    cli: &aws_sdk_dynamodb::Client,
    user_membership: &UserMembership,
    new_tier: MembershipTier,
) -> Result<()> {
    tracing::warn!("Scheduling membership downgrade to {:?}", new_tier);

    // Get the new membership details
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;

    // Schedule the downgrade by setting next_membership
    // Create updated user membership with scheduled downgrade
    let mut updated_membership = user_membership.clone();
    updated_membership.next_membership = Some(new_membership.pk.into());
    updated_membership.updated_at = now();

    // Save the scheduled downgrade (delete old, then create updated)
    updated_membership.upsert(cli).await?;

    // TODO: cancel or change payment

    tracing::info!(
        "Scheduled membership downgrade to {:?} for user {:?}, effective at {}",
        new_tier,
        user_membership.pk,
        user_membership.expired_at
    );

    Ok(())
}

/// Handle membership upgrade by immediately activating the new tier
/// Creates a purchase record and updates the user's membership
async fn handle_upgrade_membership(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    user_membership: &UserMembership,
    current_membership: Membership,
    new_tier: MembershipTier,
    card_info: Option<CardInfo>,
    currency: Currency,
) -> Result<()> {
    tracing::warn!("Processing membership upgrade to {:?}", new_tier);

    // Get the new membership details
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;
    let user_id: UserPartition = user_membership.pk.clone().into();
    let (user_payment, should_update) =
        UserPayment::get_by_user(cli, portone, user_id.clone(), card_info).await?;

    let tx_type = TransactionType::PurchaseMembership(new_tier.to_string());

    let amount = match &currency {
        Currency::Usd => new_membership.price_dollars,
        Currency::Krw => new_membership.price_won,
    };

    // Calculate prorated amount
    let remaining_duration_days = user_membership.calculate_remaining_duration_days();
    let remaining_price =
        current_membership.calculate_remaining_price(currency, remaining_duration_days);

    let amount = amount - remaining_price;

    let (res, payment_id) = portone
        .pay_with_billing_key(
            user_payment.customer_id.clone(),
            user_payment.name.clone(),
            tx_type.to_string(),
            user_payment.billing_key.clone().unwrap(),
            amount,
            currency,
        )
        .await?;

    // TODO: setup schedule period payment

    // Create a purchase record
    let user_purchase = UserPurchase::new(
        user_membership.pk.clone().into(),
        TransactionType::PurchaseMembership(new_tier.to_string()),
        match &currency {
            Currency::Usd => new_membership.price_dollars,
            Currency::Krw => new_membership.price_won,
        },
        currency,
        payment_id,
        res.payment.pg_tx_id,
    );

    let user_membership = UserMembership::new(
        user_id.clone(),
        new_membership.pk.into(),
        new_membership.duration_days,
        new_membership.credits,
    )?
    .with_purchase_id(user_purchase.pk.clone())
    .with_auto_renew(true);

    let mut txs = vec![
        user_purchase.create_transact_write_item(),
        user_membership.upsert_transact_write_item(),
    ];

    if should_update {
        txs.push(user_payment.upsert_transact_write_item());
    }

    transact_write_all_items_with_failover!(cli, txs);

    notify!(
        "Upgraded membership to {:?} for user {:?} payed by {}",
        new_tier,
        user_id,
        currency,
    );

    Ok(())
}
