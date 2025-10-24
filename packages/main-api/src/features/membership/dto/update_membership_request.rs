use crate::aide::OperationIo;
use crate::features::membership::MembershipTier;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct UpdateMembershipRequest {
    pub tier: MembershipTier,
    pub price_dollars: i64,
    pub credits: i64,
    /// Duration in days. Use -1 or 0 for infinite/lifetime memberships
    pub duration_days: i32,
    pub display_order: i32,
    pub is_active: bool,
    /// Maximum credits that can be used per space. Use -1 for unlimited
    pub max_credits_per_space: i64,
}
