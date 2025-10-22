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

    // price_dollers is a doller price of this membership
    pub price_dollers: i64,

    // Display order in the UI (lower numbers first)
    pub display_order: i32,

    // Duration in days (30 for monthly, 365 for yearly, 0 for lifetime)
    pub duration_days: i32,
}

impl Membership {
    pub fn new() -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk: Partition::Membership(uid),
            sk: EntityType::Membership,
            created_at,
            updated_at: created_at,
            ..Default::default()
        }
    }
}
