#![allow(unused_variables)]
use crate::features::membership::PaymentEntity;
use crate::features::payment::*;
use crate::models::team::{Team, TeamOwner};
use crate::services::portone::{Currency, PaymentCancelScheduleResponse, PortOne};
use crate::types::*;
use crate::*;
use aws_sdk_dynamodb::types::TransactWriteItem;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, JsonSchema, OperationIo)]
pub struct TeamPayment {
    pub pk: CompositePartition,
    pub sk: EntityType,

    pub billing_key: Option<String>,
    pub customer_id: String,
    pub name: String,
    pub birth_date: String,
}

impl TeamPayment {
    pub fn new(pk: TeamPartition, customer_id: String, name: String, birth_date: String) -> Self {
        let now = time::get_now_timestamp_millis();

        Self {
            pk: CompositePartition(pk.into(), Partition::Payment),
            sk: EntityType::Created(now.to_string()),
            customer_id,
            name,
            birth_date,

            ..Default::default()
        }
    }

    pub async fn get_by_team(
        cli: &aws_sdk_dynamodb::Client,
        team_pk: TeamPartition,
    ) -> crate::Result<Option<Self>> {
        let pk = CompositePartition::team_payment_pk(team_pk.into());
        let result: Option<TeamPayment> = Self::get(cli, &pk, None::<String>).await?;
        Ok(result)
    }

    pub async fn get_or_create(
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
        team: &Team,
        team_owner: &TeamOwner,
        card_info: Option<CardInfo>,
    ) -> crate::Result<(Self, bool)> {
        #[cfg(feature = "bypass")]
        {
            let now = time::get_now_timestamp_millis();
            let team_pk: TeamPartition = team.pk.clone().into();

            return Ok((
                Self {
                    pk: CompositePartition(team.pk.clone(), Partition::Payment),
                    sk: EntityType::Created(now.to_string()),
                    customer_id: team_pk.to_string(),
                    name: team_owner.display_name.clone(),
                    birth_date: "1990-01-15".to_string(),
                    billing_key: Some(team_pk.to_string()),
                },
                false,
            ));
        }

        #[cfg(not(feature = "bypass"))]
        {
            let pk = CompositePartition::team_payment_pk(team.pk.clone().into());
            let existing: Option<TeamPayment> = Self::get(cli, &pk, None::<String>).await?;

            let mut should_update = false;

            let mut team_payment = match existing {
                Some(payment) => payment,
                None => {
                    // Need card info to create new payment
                    let card_info = card_info.clone().ok_or(Error::CardInfoRequired)?;

                    // Create new payment record
                    let team_pk: TeamPartition = team.pk.clone().into();
                    let new_payment = TeamPayment::new(
                        team.pk.clone().into(),
                        team_pk.to_string(),
                        team_owner.display_name.clone(),
                        card_info.birth_or_business_registration_number.clone(),
                    );
                    should_update = true;
                    new_payment
                }
            };

            if team_payment.billing_key.is_none() {
                let CardInfo {
                    card_number,
                    expiry_year,
                    expiry_month,
                    birth_or_business_registration_number,
                    password_two_digits,
                } = card_info.ok_or(Error::CardInfoRequired)?;

                let res = portone
                    .get_billing_key(
                        team_payment.pk.to_string(),
                        team_payment.name.clone(),
                        card_number,
                        expiry_year,
                        expiry_month,
                        birth_or_business_registration_number,
                        password_two_digits,
                    )
                    .await?;

                team_payment.billing_key = Some(res.billing_key_info.billing_key.clone());
                should_update = true;
            }

            Ok((team_payment, should_update))
        }
    }

    pub async fn purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
    ) -> crate::Result<TeamPurchase> {
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

        let team_purchase = TeamPurchase::new(
            self.pk.0.clone().into(),
            tx_type,
            amount,
            currency,
            payment_id,
            res.payment.pg_tx_id,
        );

        Ok(team_purchase)
    }

    pub async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        time_to_pay: String,
    ) -> crate::Result<TeamPurchase> {
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

        let team_purchase = TeamPurchase::new(
            self.pk.0.clone().into(),
            tx_type,
            amount,
            currency,
            payment_id,
            res.schedule.id,
        )
        .with_status(PurchaseStatus::Scheduled);

        Ok(team_purchase)
    }

    pub async fn cancel_scheduled_payments(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
    ) -> crate::Result<PaymentCancelScheduleResponse> {
        debug!("Canceling scheduled payments for team {:?}", self);
        if self.billing_key.is_none() {
            return Err(Error::CardInfoRequired);
        }

        let res = portone
            .cancel_schedule_with_billing_key(self.billing_key.clone().unwrap())
            .await?;

        let opt = TeamPurchase::opt_one().sk(PurchaseStatus::Scheduled.to_string());
        let pk = CompositePartition::team_purchase_pk(self.pk.0.clone());
        let (purchase, _bm) = TeamPurchase::find_by_status(cli, &pk, opt).await?;
        debug!(
            "Found scheduled purchase to cancel: {:?} for {}",
            purchase, pk
        );

        let purchase = purchase
            .into_iter()
            .next()
            .ok_or(Error::InvalidIdentification)?;

        while let Err(err) = TeamPurchase::updater(&purchase.pk, &purchase.sk)
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
impl PaymentEntity for TeamPayment {
    type Purchase = TeamPurchase;

    async fn purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
    ) -> crate::Result<Self::Purchase> {
        TeamPayment::purchase(self, portone, tx_type, amount, currency).await
    }

    async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        scheduled_at: String,
    ) -> crate::Result<Self::Purchase> {
        TeamPayment::schedule_next_membership_purchase(
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
        TeamPayment::cancel_scheduled_payments(self, cli, portone).await?;
        Ok(())
    }

    fn upsert_transact_write_item(&self) -> TransactWriteItem {
        self.upsert_transact_write_item()
    }
}
