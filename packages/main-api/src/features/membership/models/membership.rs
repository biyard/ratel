use crate::features::payment::Currency;
use crate::*;
use crate::{features::membership::MembershipTier, types::*};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct Membership {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    // credits can be used for reward spaces.
    // 1 credit will be consumed to make 10,000P reward spaces.
    // This means that 10 credits is needed for 100x boosted reward space.
    pub credits: i64,

    // name is the membership name in English.
    // If translate needed, use it as i18n label in the frontend side.
    pub tier: MembershipTier,

    // Description should be defined in frontend.

    // price_dollars is a dollar price of this membership
    pub price_dollars: i64,
    pub price_won: i64,

    // Display order in the UI (lower numbers first)
    pub display_order: i32,

    // Duration in days (30 for monthly, 365 for yearly, -1 or 0 for infinite/lifetime)
    pub duration_days: i32,

    // Maximum credits that can be used per space (-1 for unlimited)
    pub max_credits_per_space: i64,

    // Is this membership currently available for purchase?
    #[dynamo(index = "gsi1", pk, prefix = "ACTIVE", name = "find_active")]
    pub is_active: bool,

    #[dynamo(index = "gsi1", sk, prefix = "ORDER")]
    pub display_order_indexed: i32,
}

impl Membership {
    pub fn new(
        tier: MembershipTier,
        price_dollers: i64,
        credits: i64,
        duration_days: i32,
        display_order: i32,
        max_credits_per_space: i64,
    ) -> Self {
        let uid = tier.to_string();
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk: Partition::Membership(uid),
            sk: EntityType::Membership,
            created_at,
            updated_at: created_at,
            tier,
            price_dollars: price_dollers,
            price_won: price_dollers * 1500,
            credits,
            duration_days,
            max_credits_per_space,
            display_order,
            is_active: true,
            display_order_indexed: display_order,
        }
    }

    /// Get membership ID from partition key
    pub fn get_id(&self) -> Option<String> {
        match &self.pk {
            Partition::Membership(id) => Some(id.clone()),
            _ => None,
        }
    }

    pub async fn get_by_membership_tier(
        cli: &aws_sdk_dynamodb::Client,
        tier: &MembershipTier,
    ) -> Result<Self> {
        let pk = Partition::Membership(tier.to_string());
        let m = Membership::get(cli, pk, Some(EntityType::Membership))
            .await?
            .ok_or_else(|| Error::NoMembershipFound)?;

        Ok(m)
    }

    pub fn calculate_remaining_price(&self, currency: Currency, remaining_days: i32) -> i64 {
        if self.duration_days <= 0 || remaining_days <= 0 || remaining_days >= self.duration_days {
            return self.price_in_currency(currency);
        }

        let daily_price = self.price_in_currency(currency) as f64 / self.duration_days as f64;
        let remaining_price = daily_price * remaining_days as f64;

        remaining_price.round() as i64
    }

    pub fn price_in_currency(&self, currency: Currency) -> i64 {
        match currency {
            Currency::Usd => self.price_dollars,
            Currency::Krw => self.price_won,
        }
    }
}
