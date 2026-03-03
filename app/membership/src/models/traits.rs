use crate::models::{
    Currency, Membership, MembershipTier, PaymentReceipt, TransactionType, UserMembership,
    UserPayment,
};
use crate::services::portone::PortOne;
use crate::*;
use aws_sdk_dynamodb::types::TransactWriteItem;

pub trait MembershipEntity: Clone + Send + Sync {
    fn owner_pk(&self) -> Partition;
    fn membership_pk(&self) -> &MembershipPartition;
    fn set_membership_pk(&mut self, pk: MembershipPartition);
    fn next_membership(&self) -> Option<&MembershipPartition>;
    fn set_next_membership(&mut self, pk: Option<MembershipPartition>);
    fn expired_at(&self) -> i64;
    fn set_updated_at(&mut self, timestamp: i64);
    fn calculate_remaining_duration_days(&self) -> i32;
    fn upsert_transact_write_item(&self) -> TransactWriteItem;

    fn is_infinite(&self) -> bool {
        self.expired_at() == i64::MAX
    }
}

#[async_trait::async_trait]
pub trait PaymentEntity: Clone + Send + Sync {
    type Purchase: PurchaseEntity;

    async fn purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
    ) -> Result<Self::Purchase>;

    async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        scheduled_at: String,
    ) -> Result<Self::Purchase>;

    async fn cancel_scheduled_payments(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
    ) -> Result<()>;

    fn upsert_transact_write_item(&self) -> TransactWriteItem;
}

pub trait PurchaseEntity: Clone + Send + Sync + Into<PaymentReceipt> {
    fn pk(&self) -> &CompositePartition;
    fn create_transact_write_item(&self) -> TransactWriteItem;
}

#[async_trait::async_trait]
impl PaymentEntity for UserPayment {
    type Purchase = crate::models::UserPurchase;

    async fn purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
    ) -> Result<Self::Purchase> {
        self.purchase(portone, tx_type, amount, currency).await
    }

    async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        scheduled_at: String,
    ) -> Result<Self::Purchase> {
        self.schedule_next_membership_purchase(portone, tx_type, amount, currency, scheduled_at)
            .await
    }

    async fn cancel_scheduled_payments(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
    ) -> Result<()> {
        self.cancel_scheduled_payments(cli, portone).await
    }

    fn upsert_transact_write_item(&self) -> TransactWriteItem {
        self.upsert_transact_write_item()
    }
}

pub async fn handle_downgrade<M, P>(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    entity_membership: &M,
    payment: Option<&P>,
    new_tier: MembershipTier,
    _owner_label: &str,
) -> Result<Membership>
where
    M: MembershipEntity,
    P: PaymentEntity,
{
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;

    let mut updated_membership = entity_membership.clone();
    updated_membership.set_next_membership(Some(new_membership.pk.clone().into()));
    updated_membership.set_updated_at(common::utils::time::now());

    if let Some(payment) = payment {
        let _ = payment.cancel_scheduled_payments(cli, portone).await;
    }

    let txs = vec![updated_membership.upsert_transact_write_item()];
    common::transact_write_all_items_with_failover!(cli, txs);

    Ok(new_membership)
}

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
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;

    let tx_type = TransactionType::PurchaseMembership(new_tier.clone());
    let amount = match currency {
        Currency::Usd => new_membership.price_dollars,
        Currency::Krw => new_membership.price_won,
    };

    let remaining_duration_days = entity_membership.calculate_remaining_duration_days();
    let remaining_price =
        current_membership.calculate_remaining_price(currency, remaining_duration_days);
    let amount = amount - remaining_price;

    let purchase = payment
        .purchase(portone, tx_type.clone(), amount, currency)
        .await?;

    let new_membership_item = new_membership_fn(&new_membership, purchase.pk())?;

    let mut txs = vec![purchase.create_transact_write_item(), new_membership_item];

    if should_update_payment {
        txs.push(payment.upsert_transact_write_item());
    }

    common::transact_write_all_items_with_failover!(cli, txs);

    Ok((purchase, new_membership))
}

impl MembershipEntity for UserMembership {
    fn owner_pk(&self) -> Partition {
        self.pk.clone()
    }

    fn membership_pk(&self) -> &MembershipPartition {
        &self.membership_pk
    }

    fn set_membership_pk(&mut self, pk: MembershipPartition) {
        self.membership_pk = pk;
    }

    fn next_membership(&self) -> Option<&MembershipPartition> {
        self.next_membership.as_ref()
    }

    fn set_next_membership(&mut self, pk: Option<MembershipPartition>) {
        self.next_membership = pk;
    }

    fn expired_at(&self) -> i64 {
        self.expired_at
    }

    fn set_updated_at(&mut self, timestamp: i64) {
        self.updated_at = timestamp;
    }

    fn calculate_remaining_duration_days(&self) -> i32 {
        self.calculate_remaining_duration_days()
    }

    fn upsert_transact_write_item(&self) -> TransactWriteItem {
        self.upsert_transact_write_item()
    }
}
