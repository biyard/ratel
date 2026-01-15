use crate::features::membership::{PaymentEntity, UserMembership};
use crate::features::payment::*;
use crate::services::portone::{Currency, PaymentCancelScheduleResponse, PortOne};
use crate::types::*;
use crate::*;
use aws_sdk_dynamodb::types::TransactWriteItem;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct UserPayment {
    pub pk: CompositePartition,
    pub sk: EntityType,

    pub billing_key: Option<String>,
    pub customer_id: String,
    pub name: String,
    pub birth_date: String,
}

impl UserPayment {
    pub fn new(pk: Partition, customer_id: String, name: String, birth_date: String) -> Self {
        if !matches!(pk, Partition::User(_)) {
            panic!("UserPayment pk must be of Partition::User type");
        }
        let now = time::get_now_timestamp_millis();

        Self {
            pk: CompositePartition(pk, Partition::Payment),
            sk: EntityType::Created(now.to_string()),
            customer_id,
            name,
            birth_date,

            ..Default::default()
        }
    }

    pub async fn get_by_user(
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
        user_pk: UserPartition,
        card_info: Option<CardInfo>,
    ) -> crate::Result<(Self, bool)> {
        #[cfg(test)]
        {
            let now = time::get_now_timestamp_millis();

            return Ok((
                Self {
                    pk: CompositePartition(user_pk.clone().into(), Partition::Payment),
                    sk: EntityType::Created(now.to_string()),
                    customer_id: user_pk.to_string(),
                    name: "Test User".to_string(),
                    birth_date: "1990-01-15".to_string(),
                    billing_key: Some(user_pk.to_string()),
                },
                false,
            ));
        }

        let pk = CompositePartition::user_payment_pk(user_pk.into());
        let mut user_payment: UserPayment = UserPayment::get(cli, &pk, None::<String>)
            .await?
            .ok_or_else(|| Error::InvalidIdentification)?;

        let mut should_update = false;

        if user_payment.billing_key.is_none() {
            let CardInfo {
                card_number,
                expiry_year,
                expiry_month,
                birth_or_business_registration_number,
                password_two_digits,
            } = card_info.ok_or_else(|| Error::CardInfoRequired)?;

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

            user_payment.billing_key = Some(res.billing_key_info.billing_key.clone());

            should_update = true;
        }

        Ok((user_payment, should_update))
    }

    pub async fn purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
    ) -> crate::Result<UserPurchase> {
        let (res, payment_id) = portone
            .pay_with_billing_key(
                self.customer_id.clone(),
                self.name.clone(),
                tx_type.to_string(),
                self.billing_key.clone().unwrap(),
                amount,
                currency,
            )
            .await?;

        let user_purchase = UserPurchase::new(
            self.pk.0.clone().into(),
            tx_type,
            amount,
            currency,
            payment_id,
            res.payment.pg_tx_id,
        );

        Ok(user_purchase)
    }

    pub async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        time_to_pay: String,
    ) -> crate::Result<UserPurchase> {
        let (res, payment_id) = portone
            .schedule_pay_with_billing_key(
                self.customer_id.clone(),
                self.name.clone(),
                tx_type.to_string(),
                self.billing_key.clone().unwrap(),
                amount,
                currency,
                time_to_pay,
            )
            .await?;

        let user_purchase = UserPurchase::new(
            self.pk.0.clone().into(),
            tx_type,
            amount,
            currency,
            payment_id,
            res.schedule.id,
        )
        .with_status(PurchaseStatus::Scheduled);

        Ok(user_purchase)
    }

    pub async fn cancel_scheduled_payments(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
    ) -> crate::Result<PaymentCancelScheduleResponse> {
        debug!("Canceling scheduled payments for user {:?}", self);
        if self.billing_key.is_none() {
            return Err(Error::CardInfoRequired);
        }

        let res = portone
            .cancel_schedule_with_billing_key(self.billing_key.clone().unwrap())
            .await?;

        let opt = UserPurchase::opt_one().sk(PurchaseStatus::Scheduled.to_string());
        let pk = CompositePartition::user_purchase_pk(self.pk.0.clone());
        let (purchase, _bm) = UserPurchase::find_by_status(cli, &pk, opt).await?;
        debug!(
            "Found scheduled purchase to cancel: {:?} for {}",
            purchase, pk
        );

        let purchase = purchase
            .into_iter()
            .next()
            .ok_or_else(|| Error::InvalidIdentification)?;

        while let Err(err) = UserPurchase::updater(&purchase.pk, &purchase.sk)
            .with_status(PurchaseStatus::Canceled)
            .execute(cli)
            .await
        {
            error!(
                "Failed to update purchase status to Canceled: {:?}, retrying...",
                err
            );
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        Ok(res)
    }
}
#[async_trait::async_trait]
impl PaymentEntity for UserPayment {
    type Purchase = UserPurchase;

    async fn purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
    ) -> crate::Result<Self::Purchase> {
        UserPayment::purchase(self, portone, tx_type, amount, currency).await
    }

    async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        scheduled_at: String,
    ) -> crate::Result<Self::Purchase> {
        UserPayment::schedule_next_membership_purchase(
            self,
            portone,
            tx_type,
            amount,
            currency,
            scheduled_at,
        )
        .await
    }

    async fn cancel_scheduled_payments(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
    ) -> crate::Result<()> {
        UserPayment::cancel_scheduled_payments(self, cli, portone).await?;
        Ok(())
    }

    fn upsert_transact_write_item(&self) -> TransactWriteItem {
        self.upsert_transact_write_item()
    }
}
