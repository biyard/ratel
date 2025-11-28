use crate::features::membership::*;
use crate::*;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct MembershipResponse {
    pub id: MembershipPartition,
    pub tier: MembershipTier,
    pub price_dollars: i64,
    pub credits: i64,
    /// Duration in days. -1 or 0 indicates infinite/lifetime membership
    pub duration_days: i32,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    /// Maximum credits that can be used per space. -1 indicates unlimited
    pub max_credits_per_space: i64,
}

impl From<Membership> for MembershipResponse {
    fn from(membership: Membership) -> Self {
        Self {
            id: membership.pk.into(),
            tier: membership.tier,
            price_dollars: membership.price_dollars,
            credits: membership.credits,
            duration_days: membership.duration_days,
            display_order: membership.display_order,
            is_active: membership.is_active,
            created_at: membership.created_at,
            updated_at: membership.updated_at,
            max_credits_per_space: membership.max_credits_per_space,
        }
    }
}
