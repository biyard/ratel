use crate::features::membership::models::MembershipTier;
use crate::features::membership::*;

#[cfg(feature = "server")]
use crate::features::auth::utils::uuid::sorted_uuid;
#[cfg(feature = "server")]
use crate::features::membership::models::PurchaseEntity;
#[cfg(feature = "server")]
use crate::features::membership::services::portone::PortOne;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CardInfo {
    pub card_number: String,
    pub expiry_year: String,
    pub expiry_month: String,
    pub birth_or_business_registration_number: String,
    pub password_two_digits: String,
}

#[derive(Debug, Clone, Copy, Default, SerializeDisplay, DeserializeFromStr)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Currency {
    #[default]
    Usd,
    Krw,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Currency::Usd => "USD",
            Currency::Krw => "KRW",
        };
        f.write_str(value)
    }
}

impl std::str::FromStr for Currency {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "USD" => Ok(Currency::Usd),
            "KRW" => Ok(Currency::Krw),
            _ => {
                crate::error!("invalid currency: {s}");
                Err(crate::features::membership::types::MembershipPaymentError::InvalidCurrency.into())
            }
        }
    }
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEnum, Eq, PartialEq,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum PurchaseStatus {
    #[default]
    Success,
    Scheduled,
    Canceled,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Default, DynamoEnum)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum TransactionType {
    #[default]
    None,
    PurchaseMembership(MembershipTier),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PaymentReceipt {
    pub id: String,
    pub paid_at: i64,
    pub tx_type: TransactionType,
    pub currency: Currency,
    pub tx_id: String,
    pub amount: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserPurchase {
    #[dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", pk)]
    #[dynamo(prefix = "PAYMENT", name = "find_by_status", index = "gsi2", pk)]
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[dynamo(prefix = "PAYMENT", name = "find_by_user", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(name = "find_by_status", index = "gsi2", sk)]
    #[dynamo(name = "find_by_payment_id", index = "gsi3", sk)]
    pub status: PurchaseStatus,
    pub tx_type: TransactionType,
    pub amount: i64,
    pub currency: Currency,
    #[dynamo(prefix = "PAYMENT", name = "find_by_payment_id", index = "gsi3", pk)]
    pub payment_id: String,
    pub tx_id: String,
}

#[cfg(feature = "server")]
impl UserPurchase {
    pub fn new(
        pk: UserPartition,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        payment_id: String,
        tx_id: String,
    ) -> Self {
        let uuid = sorted_uuid();
        let created_at = crate::common::utils::time::now();

        Self {
            pk: CompositePartition::user_purchase_pk(pk.into()),
            sk: EntityType::UserPurchase(uuid),
            tx_type,
            amount,
            created_at,
            payment_id,
            tx_id,
            currency,
            status: PurchaseStatus::Success,
        }
    }
}

impl From<UserPurchase> for PaymentReceipt {
    fn from(purchase: UserPurchase) -> Self {
        Self {
            id: purchase.payment_id,
            paid_at: purchase.created_at,
            tx_type: purchase.tx_type,
            currency: purchase.currency,
            tx_id: purchase.tx_id,
            amount: purchase.amount,
        }
    }
}

#[cfg(feature = "server")]
impl PurchaseEntity for UserPurchase {
    fn pk(&self) -> &CompositePartition {
        &self.pk
    }

    fn create_transact_write_item(&self) -> aws_sdk_dynamodb::types::TransactWriteItem {
        self.create_transact_write_item()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserPayment {
    pub pk: CompositePartition,
    pub sk: EntityType,

    pub billing_key: Option<String>,
    pub customer_id: String,
    pub name: String,
    pub birth_date: String,
    #[serde(default)]
    pub masked_card_number: Option<String>,
}

#[cfg(feature = "server")]
impl UserPayment {
    pub fn new(pk: Partition, customer_id: String, name: String, birth_date: String) -> Self {
        let now = crate::common::utils::time::now();
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
    ) -> Result<(Self, bool)> {
        let pk = CompositePartition::user_payment_pk(user_pk.into());
        let mut user_payment: UserPayment = UserPayment::get(cli, &pk, None::<String>)
            .await?
            .ok_or_else(|| Error::NotFound("User payment not found".to_string()))?;

        let mut should_update = false;

        if user_payment.billing_key.is_none() {
            let CardInfo {
                card_number,
                expiry_year,
                expiry_month,
                birth_or_business_registration_number,
                password_two_digits,
            } = card_info.ok_or_else(|| Error::from(crate::features::membership::types::MembershipPaymentError::MissingCardInfo))?;

            // Store masked card number before passing to Portone
            let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();
            let last4 = if digits.len() >= 4 {
                &digits[digits.len() - 4..]
            } else {
                "****"
            };
            user_payment.masked_card_number = Some(format!("****-****-****-{last4}"));

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
    ) -> Result<UserPurchase> {
        let (res, payment_id) = portone
            .pay_with_billing_key(
                self.customer_id.clone(),
                self.name.clone(),
                tx_type.to_string(),
                self.billing_key.clone().unwrap_or_default(),
                amount,
                currency,
            )
            .await?;

        Ok(UserPurchase::new(
            self.pk.0.clone().into(),
            tx_type,
            amount,
            currency,
            payment_id,
            res.payment.pg_tx_id,
        ))
    }

    pub async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        time_to_pay: String,
    ) -> Result<UserPurchase> {
        let (res, payment_id) = portone
            .schedule_pay_with_billing_key(
                self.customer_id.clone(),
                self.name.clone(),
                tx_type.to_string(),
                self.billing_key.clone().unwrap_or_default(),
                amount,
                currency,
                time_to_pay,
            )
            .await?;

        Ok(UserPurchase::new(
            self.pk.0.clone().into(),
            tx_type,
            amount,
            currency,
            payment_id,
            res.schedule.id,
        )
        .with_status(PurchaseStatus::Scheduled))
    }

    pub async fn cancel_scheduled_payments(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
    ) -> Result<()> {
        if self.billing_key.is_none() {
            return Err(crate::features::membership::types::MembershipPaymentError::MissingBillingKey.into());
        }

        let _ = portone
            .cancel_schedule_with_billing_key(self.billing_key.clone().unwrap())
            .await?;

        let opt = UserPurchase::opt_one().sk(PurchaseStatus::Scheduled.to_string());
        let pk = CompositePartition::user_purchase_pk(self.pk.0.clone());
        let (purchase, _bm) = UserPurchase::find_by_status(cli, &pk, opt).await?;

        let purchase = match purchase.into_iter().next() {
            Some(item) => item,
            None => return Ok(()),
        };

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

        Ok(())
    }
}

// ── Team Payment & Purchase ──────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamPurchase {
    #[dynamo(prefix = "PAYMENT", name = "find_by_team", index = "gsi1", pk)]
    #[dynamo(prefix = "PAYMENT", name = "find_by_status", index = "gsi2", pk)]
    pub pk: CompositePartition,
    pub sk: EntityType,

    #[dynamo(prefix = "PAYMENT", name = "find_by_team", index = "gsi1", sk)]
    pub created_at: i64,

    #[dynamo(name = "find_by_status", index = "gsi2", sk)]
    #[dynamo(name = "find_by_payment_id", index = "gsi3", sk)]
    pub status: PurchaseStatus,
    pub tx_type: TransactionType,
    pub amount: i64,
    pub currency: Currency,
    #[dynamo(prefix = "PAYMENT", name = "find_by_payment_id", index = "gsi3", pk)]
    pub payment_id: String,
    pub tx_id: String,
}

#[cfg(feature = "server")]
impl TeamPurchase {
    pub fn new(
        pk: TeamPartition,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        payment_id: String,
        tx_id: String,
    ) -> Self {
        let uuid = crate::features::auth::utils::uuid::sorted_uuid();
        let created_at = crate::common::utils::time::now();

        Self {
            pk: CompositePartition::team_purchase_pk(pk.into()),
            sk: EntityType::TeamPurchase(uuid),
            tx_type,
            amount,
            created_at,
            payment_id,
            tx_id,
            currency,
            status: PurchaseStatus::Success,
        }
    }
}

impl From<TeamPurchase> for PaymentReceipt {
    fn from(purchase: TeamPurchase) -> Self {
        Self {
            id: purchase.payment_id,
            paid_at: purchase.created_at,
            tx_type: purchase.tx_type,
            currency: purchase.currency,
            tx_id: purchase.tx_id,
            amount: purchase.amount,
        }
    }
}

#[cfg(feature = "server")]
impl PurchaseEntity for TeamPurchase {
    fn pk(&self) -> &CompositePartition {
        &self.pk
    }

    fn create_transact_write_item(&self) -> aws_sdk_dynamodb::types::TransactWriteItem {
        self.create_transact_write_item()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamPayment {
    pub pk: CompositePartition,
    pub sk: EntityType,

    pub billing_key: Option<String>,
    pub customer_id: String,
    pub name: String,
    pub birth_date: String,
    #[serde(default)]
    pub masked_card_number: Option<String>,
}

#[cfg(feature = "server")]
impl TeamPayment {
    pub fn new(pk: Partition, customer_id: String, name: String, birth_date: String) -> Self {
        let now = crate::common::utils::time::now();
        Self {
            pk: CompositePartition(pk, Partition::Payment),
            sk: EntityType::Created(now.to_string()),
            customer_id,
            name,
            birth_date,
            ..Default::default()
        }
    }

    pub async fn get_by_team(
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
        team_pk: TeamPartition,
        card_info: Option<CardInfo>,
    ) -> Result<(Self, bool)> {
        let pk = CompositePartition::team_payment_pk(team_pk.into());
        let mut team_payment: TeamPayment = TeamPayment::get(cli, &pk, None::<String>)
            .await?
            .ok_or_else(|| Error::NotFound("Team payment not found".to_string()))?;

        let mut should_update = false;

        if team_payment.billing_key.is_none() {
            let CardInfo {
                card_number,
                expiry_year,
                expiry_month,
                birth_or_business_registration_number,
                password_two_digits,
            } = card_info.ok_or_else(|| Error::from(crate::features::membership::types::MembershipPaymentError::MissingCardInfo))?;

            let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();
            let last4 = if digits.len() >= 4 {
                &digits[digits.len() - 4..]
            } else {
                "****"
            };
            team_payment.masked_card_number = Some(format!("****-****-****-{last4}"));

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

    pub async fn purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
    ) -> Result<TeamPurchase> {
        let (res, payment_id) = portone
            .pay_with_billing_key(
                self.customer_id.clone(),
                self.name.clone(),
                tx_type.to_string(),
                self.billing_key.clone().unwrap_or_default(),
                amount,
                currency,
            )
            .await?;

        Ok(TeamPurchase::new(
            self.pk.0.clone().into(),
            tx_type,
            amount,
            currency,
            payment_id,
            res.payment.pg_tx_id,
        ))
    }

    pub async fn schedule_next_membership_purchase(
        &self,
        portone: &PortOne,
        tx_type: TransactionType,
        amount: i64,
        currency: Currency,
        time_to_pay: String,
    ) -> Result<TeamPurchase> {
        let (res, payment_id) = portone
            .schedule_pay_with_billing_key(
                self.customer_id.clone(),
                self.name.clone(),
                tx_type.to_string(),
                self.billing_key.clone().unwrap_or_default(),
                amount,
                currency,
                time_to_pay,
            )
            .await?;

        Ok(TeamPurchase::new(
            self.pk.0.clone().into(),
            tx_type,
            amount,
            currency,
            payment_id,
            res.schedule.id,
        )
        .with_status(PurchaseStatus::Scheduled))
    }

    pub async fn cancel_scheduled_payments(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        portone: &PortOne,
    ) -> Result<()> {
        if self.billing_key.is_none() {
            return Err(crate::features::membership::types::MembershipPaymentError::MissingBillingKey.into());
        }

        let _ = portone
            .cancel_schedule_with_billing_key(self.billing_key.clone().unwrap())
            .await?;

        let opt = TeamPurchase::opt_one().sk(PurchaseStatus::Scheduled.to_string());
        let pk = CompositePartition::team_purchase_pk(self.pk.0.clone());
        let (purchase, _bm) = TeamPurchase::find_by_status(cli, &pk, opt).await?;

        let purchase = match purchase.into_iter().next() {
            Some(item) => item,
            None => return Ok(()),
        };

        while let Err(err) = TeamPurchase::updater(&purchase.pk, &purchase.sk)
            .with_status(PurchaseStatus::Canceled)
            .execute(cli)
            .await
        {
            error!(
                "Failed to update team purchase status to Canceled: {:?}, retrying...",
                err
            );
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }
}
