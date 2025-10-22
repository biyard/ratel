use crate::aide::OperationIo;
use crate::features::membership::dto::MembershipResponse;
use bdk::prelude::*;

#[derive(
    Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo
)]
pub struct ListMembershipsResponse {
    pub memberships: Vec<MembershipResponse>,
    pub total: usize,
}
