use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
pub struct UserMembershipLocal {
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub updated_at: i64,
    pub expired_at: i64,
    pub membership_pk: MembershipPartition,
    pub status: String,
    pub total_credits: i64,
    pub remaining_credits: i64,
    pub auto_renew: bool,
    pub next_membership: Option<MembershipPartition>,
    pub monthly_refill_credits: i64,
    pub next_refill_at: i64,
}
