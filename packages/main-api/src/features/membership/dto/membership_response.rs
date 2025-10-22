use crate::aide::OperationIo;
use crate::features::membership::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo
)]
pub struct MembershipResponse {
    pub id: String,
    pub tier: MembershipTier,
    pub price_dollers: i64,
    pub credits: i64,
    pub duration_days: i32,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Membership> for MembershipResponse {
    fn from(membership: Membership) -> Self {
        Self {
            id: membership.get_id().unwrap_or_default(),
            tier: membership.tier,
            price_dollers: membership.price_dollers,
            credits: membership.credits,
            duration_days: membership.duration_days,
            display_order: membership.display_order,
            is_active: membership.is_active,
            created_at: membership.created_at,
            updated_at: membership.updated_at,
        }
    }
}
