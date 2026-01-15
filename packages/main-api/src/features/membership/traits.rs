use crate::{
    features::{
        membership::{Membership, MembershipTier},
        payment::{PaymentReceipt, TransactionType},
    },
    services::portone::{Currency, PortOne},
    types::{CompositePartition, MembershipPartition, Partition},
    Result,
};
use aws_sdk_dynamodb::types::TransactWriteItem;

/// Trait for membership entities (UserMembership, TeamMembership)
/// Provides common interface for membership operations
pub trait MembershipEntity: Clone + Send + Sync {
    /// Get the primary key of the membership owner
    fn owner_pk(&self) -> Partition;

    /// Get the membership plan partition key
    fn membership_pk(&self) -> &MembershipPartition;

    /// Set the membership plan partition key
    fn set_membership_pk(&mut self, pk: MembershipPartition);

    /// Get the next scheduled membership (for downgrades)
    fn next_membership(&self) -> Option<&MembershipPartition>;

    /// Set the next scheduled membership
    fn set_next_membership(&mut self, pk: Option<MembershipPartition>);

    /// Get the expiration timestamp
    fn expired_at(&self) -> i64;

    /// Set the updated_at timestamp
    fn set_updated_at(&mut self, timestamp: i64);

    /// Calculate remaining duration in days
    fn calculate_remaining_duration_days(&self) -> i32;

    /// Create a transact write item for upserting
    fn upsert_transact_write_item(&self) -> TransactWriteItem;

    /// Check if membership has infinite duration
    fn is_infinite(&self) -> bool {
        self.expired_at() == i64::MAX
    }

    /// Check if membership is expired
    fn is_expired(&self) -> bool {
        self.expired_at() != i64::MAX && self.expired_at() <= crate::now()
    }
}

/// Trait for payment entities (UserPayment, TeamPayment)
pub trait PaymentEntity: Clone + Send + Sync {
    type Purchase: PurchaseEntity;

    /// Execute a purchase
    async fn purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
    ) -> Result<Self::Purchase>;

    /// Schedule next membership purchase
    async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        scheduled_at: String,
    ) -> Result<Self::Purchase>;

    /// Cancel scheduled payments
    async fn cancel_scheduled_payments(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
    ) -> Result<()>;

    /// Create a transact write item for upserting
    fn upsert_transact_write_item(&self) -> TransactWriteItem;
}

/// Trait for purchase entities (UserPurchase, TeamPurchase)
pub trait PurchaseEntity: Clone + Send + Sync + Into<PaymentReceipt> {
    /// Get the composite partition key
    fn pk(&self) -> &CompositePartition;

    /// Create a transact write item for creating
    fn create_transact_write_item(&self) -> TransactWriteItem;
}

/// Handle membership downgrade by scheduling it for next renewal
pub async fn handle_downgrade<M, P>(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    entity_membership: &M,
    payment: Option<&P>,
    new_tier: MembershipTier,
    owner_label: &str,
) -> Result<Membership>
where
    M: MembershipEntity,
    P: PaymentEntity,
{
    tracing::debug!("Scheduling membership downgrade to {:?}", new_tier);

    // Get the new membership details
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;

    // Schedule the downgrade by setting next_membership
    let mut updated_membership = entity_membership.clone();
    updated_membership.set_next_membership(Some(new_membership.pk.clone().into()));
    updated_membership.set_updated_at(crate::now());

    // Cancel any scheduled payments
    if let Some(payment) = payment {
        let _ = payment.cancel_scheduled_payments(cli, portone).await;
    }

    // Save the scheduled downgrade
    use crate::transact_write_all_items_with_failover;
    let txs = vec![updated_membership.upsert_transact_write_item()];
    transact_write_all_items_with_failover!(cli, txs);

    tracing::info!(
        "Scheduled membership downgrade to {:?} for {} {:?}, effective at {}",
        new_tier,
        owner_label,
        entity_membership.owner_pk(),
        entity_membership.expired_at()
    );

    Ok(new_membership)
}

/// Handle membership upgrade by immediately activating the new tier
pub async fn handle_upgrade<M, P, NewMembershipFn>(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    entity_membership: &M,
    current_membership: Membership,
    new_tier: MembershipTier,
    currency: Currency,
    payment: P,
    should_update_payment: bool,
    new_membership_fn: NewMembershipFn,
) -> Result<(P::Purchase, Membership)>
where
    M: MembershipEntity,
    P: PaymentEntity,
    NewMembershipFn: FnOnce(&Membership, &CompositePartition) -> Result<TransactWriteItem>,
{
    use crate::transact_write_all_items_with_failover;
    use crate::utils::time::after_days_from_now_rfc_3339;

    tracing::debug!("Processing membership upgrade to {:?}", new_tier);

    // Get the new membership details
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;

    let tx_type = TransactionType::PurchaseMembership(new_tier.clone());

    let amount = match &currency {
        Currency::Usd => new_membership.price_dollars,
        Currency::Krw => new_membership.price_won,
    };

    // Calculate prorated amount
    let remaining_duration_days = entity_membership.calculate_remaining_duration_days();
    let remaining_price =
        current_membership.calculate_remaining_price(currency.clone(), remaining_duration_days);

    let amount = amount - remaining_price;

    // Create a purchase record
    let purchase = payment
        .purchase(portone, tx_type.clone(), amount, currency.clone())
        .await?;

    // Build transaction items
    let new_membership_item = new_membership_fn(&new_membership, purchase.pk())?;

    let mut txs = vec![purchase.create_transact_write_item(), new_membership_item];

    #[cfg(test)]
    {
        // NOTE: Real payment will call /hooks/portone but testing code.
        let next_time_to_pay = after_days_from_now_rfc_3339(new_membership.duration_days as i64);
        let next_purchase = payment
            .schedule_next_membership_purchase(
                portone,
                tx_type,
                amount,
                currency,
                next_time_to_pay,
            )
            .await?;
        txs.push(next_purchase.create_transact_write_item());
    }

    if should_update_payment {
        txs.push(payment.upsert_transact_write_item());
    }

    transact_write_all_items_with_failover!(cli, txs);

    Ok((purchase, new_membership))
}
