use crate::aide::OperationIo;
use crate::features::membership::*;
use crate::types::Partition;
use bdk::prelude::*;

#[derive(
    Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo
)]
pub struct UserMembershipResponse {
    pub user_id: String,
    pub membership_id: String,
    pub status: MembershipStatus,
    pub total_credits: i64,
    pub remaining_credits: i64,
    pub auto_renew: bool,
    pub renewal_count: i32,
    pub price_paid: i64,
    pub transaction_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub expired_at: i64,
    pub cancelled_at: Option<i64>,
    pub cancellation_reason: Option<String>,
}

impl From<UserMembership> for UserMembershipResponse {
    fn from(user_membership: UserMembership) -> Self {
        let user_id = match user_membership.pk {
            Partition::User(id) => id,
            _ => String::new(),
        };

        let membership_id = match user_membership.membership_pk {
            Partition::Membership(id) => id,
            _ => String::new(),
        };

        Self {
            user_id,
            membership_id,
            status: user_membership.status,
            total_credits: user_membership.total_credits,
            remaining_credits: user_membership.remaining_credits,
            auto_renew: user_membership.auto_renew,
            renewal_count: user_membership.renewal_count,
            price_paid: user_membership.price_paid,
            transaction_id: user_membership.transaction_id,
            created_at: user_membership.created_at,
            updated_at: user_membership.updated_at,
            expired_at: user_membership.expired_at,
            cancelled_at: user_membership.cancelled_at,
            cancellation_reason: user_membership.cancellation_reason,
        }
    }
}
