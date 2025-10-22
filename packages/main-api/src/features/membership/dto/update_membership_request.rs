use crate::aide::OperationIo;
use crate::features::membership::MembershipTier;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct UpdateMembershipRequest {
    pub tier: Option<MembershipTier>,
    pub price_dollers: Option<i64>,
    pub credits: Option<i64>,
    pub duration_days: Option<i32>,
    pub display_order: Option<i32>,
    pub is_active: Option<bool>,
}
