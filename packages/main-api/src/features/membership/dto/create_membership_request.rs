use crate::aide::OperationIo;
use crate::features::membership::MembershipTier;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct CreateMembershipRequest {
    pub tier: MembershipTier,
    pub price_dollers: i64,
    pub credits: i64,
    pub duration_days: i32,
    pub display_order: i32,
}
