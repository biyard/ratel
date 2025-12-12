use crate::{features::membership::UserMembership, *};

#[derive(Debug, Clone, Serialize, Deserialize, Default, JsonSchema, OperationIo)]
pub struct UserMembershipResponse {
    pub tier: MembershipPartition,

    pub expired_at: i64,

    pub total_credits: i64,
    pub remaining_credits: i64,

    pub next_membership: Option<MembershipPartition>,
}

impl From<UserMembership> for UserMembershipResponse {
    fn from(user_membership: UserMembership) -> Self {
        Self {
            tier: user_membership.membership_pk,
            expired_at: user_membership.expired_at,
            total_credits: user_membership.total_credits,
            remaining_credits: user_membership.remaining_credits,
            next_membership: user_membership.next_membership,
        }
    }
}
