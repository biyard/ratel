use crate::features::membership::*;
use crate::features::payment::*;
use crate::services::portone::{Currency, PortOne};
use crate::utils::time::after_days_from_now_rfc_3339;

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
    pub renewal_date: i64,
    pub receipt: Option<PaymentReceipt>,
    pub membership: Option<MembershipResponse>,
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

    let mut ret = ChangeMembershipResponse {
        renewal_date: now(),
        receipt: None,
        membership: None,
    };

    if req.membership < current_membership.tier {
        let membership =
            handle_downgrade_membership(cli, &portone, &user_membership, req.membership.clone())
                .await?;

        ret.renewal_date = user_membership.expired_at + 1;
        ret.membership = Some(membership.into());
    } else {
        let (user_purchase, membership) = handle_upgrade_membership(
            cli,
            &portone,
            &user_membership,
            current_membership,
            req.membership.clone(),
            req.card_info,
            req.currency,
        )
        .await?;

        ret.receipt = Some(user_purchase.into());
        ret.membership = Some(membership.into());
    };

    Ok(Json(ret))
}

/// Handle membership downgrade by scheduling it for next renewal
/// The downgrade takes effect when current membership expires
async fn handle_downgrade_membership(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    user_membership: &UserMembership,
    new_tier: MembershipTier,
) -> Result<Membership> {
    tracing::warn!("Scheduling membership downgrade to {:?}", new_tier);

    // Get the new membership details
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;

    // Schedule the downgrade by setting next_membership
    // Create updated user membership with scheduled downgrade
    let mut updated_membership = user_membership.clone();
    updated_membership.next_membership = Some(new_membership.pk.clone().into());
    updated_membership.updated_at = now();

    let user_id: UserPartition = user_membership.pk.clone().into();
    let (user_payment, _) = UserPayment::get_by_user(cli, portone, user_id.clone(), None).await?;
    user_payment.cancel_scheduled_payments(cli, portone).await?;

    // Save the scheduled downgrade (delete old, then create updated)
    updated_membership.upsert(cli).await?;

    notify!(
        "Scheduled membership downgrade to {:?} for user {:?}, effective at {}",
        new_tier,
        user_membership.pk,
        user_membership.expired_at
    );

    Ok(new_membership)
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
) -> Result<(UserPurchase, Membership)> {
    tracing::debug!("Processing membership upgrade to {:?}", new_tier);

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

    // Create a purchase record
    let user_purchase = user_payment
        .purchase(portone, tx_type.clone(), amount, currency)
        .await?;

    let mut user_membership = UserMembership::new(
        user_id.clone(),
        new_membership.pk.clone().into(),
        new_membership.duration_days,
        new_membership.credits,
    )?
    .with_purchase_id(user_purchase.pk.clone())
    .with_auto_renew(true);

    let next_purchase = user_payment
        .schedule_next_membership_purchase(
            portone,
            tx_type,
            amount,
            currency,
            user_membership
                .renewal_date_rfc_3339()
                .unwrap_or(after_days_from_now_rfc_3339(
                    new_membership.duration_days as i64,
                )),
        )
        .await;

    if next_purchase.is_err() {
        user_membership.auto_renew = false;
        user_membership.next_membership =
            Some(Partition::Membership(MembershipTier::Free.to_string()).into())
    }

    let mut txs = vec![
        user_purchase.create_transact_write_item(),
        user_membership.upsert_transact_write_item(),
    ];

    if let Ok(next_purchase) = next_purchase {
        txs.push(next_purchase.create_transact_write_item());
    }

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

    Ok((user_purchase, new_membership))
}
