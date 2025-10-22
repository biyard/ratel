use crate::{features::membership::MembershipTier, types::*};
use bdk::prelude::*;

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

    // Display order in the UI (lower numbers first)
    pub display_order: i32,

    // Duration in days (30 for monthly, 365 for yearly, 0 for lifetime)
    pub duration_days: i32,

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
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk: Partition::Membership(uid),
            sk: EntityType::Membership,
            created_at,
            updated_at: created_at,
            tier,
            price_dollars: price_dollers,
            credits,
            duration_days,
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

    /// Update membership fields
    pub fn update(
        &mut self,
        tier: Option<MembershipTier>,
        price_dollers: Option<i64>,
        credits: Option<i64>,
        duration_days: Option<i32>,
        display_order: Option<i32>,
        is_active: Option<bool>,
    ) {
        if let Some(tier) = tier {
            self.tier = tier;
        }
        if let Some(price) = price_dollers {
            self.price_dollars = price;
        }
        if let Some(credits) = credits {
            self.credits = credits;
        }
        if let Some(duration) = duration_days {
            self.duration_days = duration;
        }
        if let Some(order) = display_order {
            self.display_order = order;
            self.display_order_indexed = order;
        }
        if let Some(active) = is_active {
            self.is_active = active;
        }
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }
}
