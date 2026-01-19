use crate::features::membership::{MembershipResponse, MembershipTier};
use crate::features::payment::{CardInfo, PaymentReceipt};
use crate::services::portone::Currency;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct ChangeTeamMembershipRequest {
    #[schemars(description = "Membership tier to be paid for")]
    pub membership: MembershipTier,
    pub currency: Currency,
    pub card_info: Option<CardInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema, Default)]
pub struct ChangeTeamMembershipResponse {
    #[schemars(description = "Status of the operation")]
    #[serde(default)]
    pub renewal_date: i64,
    pub receipt: Option<PaymentReceipt>,
    pub membership: Option<MembershipResponse>,
}
