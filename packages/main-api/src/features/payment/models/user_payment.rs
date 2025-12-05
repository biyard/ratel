use crate::features::membership::UserMembership;
use crate::features::payment::*;
use crate::services::portone::PortOne;
use crate::types::*;
use crate::*;

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
            res.payment.pg_tx_id,
        )
        .with_status(PurchaseStatus::Scheduled);

        Ok(user_purchase)
    }
}
