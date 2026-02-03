use crate::{
    features::membership::{MembershipEntity, MembershipStatus},
    types::*,
    *,
};
use aws_sdk_dynamodb::types::TransactWriteItem;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct UserMembership {
    pub pk: Partition,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,
    pub expired_at: i64,

    #[dynamo(prefix = "UM", name = "find_by_membership", index = "gsi1", pk)]
    pub membership_pk: MembershipPartition,
    pub status: MembershipStatus,

    // Credits management
    pub total_credits: i64,
    pub remaining_credits: i64,

    // Renewal tracking
    pub auto_renew: bool,

    // Optional: Next membership tier for auto-renewal
    // If absent, keep same membership
    pub next_membership: Option<MembershipPartition>,
}

impl UserMembership {
    pub fn new(
        user_pk: UserPartition,
        membership_pk: MembershipPartition,
        duration_days: i32,
        credits: i64,
    ) -> crate::Result<Self> {
        let created_at = now();

        // Fix: Convert to i64 before multiplication to prevent overflow
        // Support -1 and 0 for infinite/lifetime memberships
        let expired_at = if duration_days <= 0 {
            // Infinite/Lifetime membership (far future)
            i64::MAX
        } else {
            created_at + (duration_days as i64) * config::get().day_unit()
        };

        Ok(Self {
            pk: user_pk.into(),
            sk: EntityType::UserMembership,
            membership_pk,
            created_at,
            updated_at: created_at,
            expired_at,
            total_credits: credits,
            remaining_credits: credits,
            auto_renew: true,
            status: MembershipStatus::Active,
            next_membership: None,
        })
    }

    // FIXME: check membership paid check logic
    pub fn is_paid_membership(&self) -> bool {
        let membership_name = self.membership_pk.to_string();
        !(membership_name.contains("Free") || membership_name.contains("FREE"))
    }

    /// Check if membership is currently active and not expired
    pub fn is_active(&self) -> bool {
        self.status == MembershipStatus::Active
            && (self.expired_at == i64::MAX || self.expired_at > now())
    }

    /// Check if membership is expired
    pub fn is_expired(&self) -> bool {
        // Infinite memberships (i64::MAX) never expire
        self.expired_at != i64::MAX && self.expired_at <= now()
    }

    /// Check if membership has infinite duration
    pub fn is_infinite(&self) -> bool {
        self.expired_at == i64::MAX
    }

    /// Use credits from this membership
    pub fn use_credits(&mut self, amount: i64) -> crate::Result<()> {
        if self.remaining_credits < amount {
            return Err(crate::Error::InsufficientCredits);
        }

        self.remaining_credits -= amount;
        self.updated_at = now();

        Ok(())
    }

    /// Add credits to this membership
    pub fn add_credits(&mut self, amount: i64) {
        self.remaining_credits += amount;
        self.total_credits += amount;
        self.updated_at = now();
    }

    /// Mark as expired if past expiration date
    pub fn check_and_update_expiration(&mut self) -> bool {
        if self.is_expired() && self.status == MembershipStatus::Active {
            self.status = MembershipStatus::Expired;
            self.updated_at = now();
            true
        } else {
            false
        }
    }

    /// Get membership status as enum
    pub fn get_status(&self) -> MembershipStatus {
        MembershipStatus::from(self.status.clone())
    }

    /// Set membership status
    pub fn set_status(&mut self, status: MembershipStatus) {
        self.status = status.into();
        self.updated_at = now();
    }

    /// Builder method - placeholder for purchase_id (field doesn't exist in current model)
    /// This is a compatibility method for existing code
    pub fn with_purchase_id(self, _purchase_id: CompositePartition) -> Self {
        // No-op: purchase_id field doesn't exist in the simplified model
        self
    }

    pub fn calculate_remaining_duration_days(&self) -> i32 {
        if self.is_infinite() {
            return -1; // Infinite duration
        }

        let now = now();
        if self.expired_at <= now {
            return 0; // Already expired
        }

        let remaining_millis = self.expired_at - now;
        let remaining_days = remaining_millis / config::get().day_unit();

        remaining_days as i32
    }

    pub fn renewal_date_rfc_3339(&self) -> Option<String> {
        if self.is_infinite() {
            return None; // Infinite memberships do not have a renewal date
        }

        let datetime = chrono::DateTime::from_timestamp_millis(self.expired_at).unwrap();
        Some(datetime.to_rfc3339())
    }
}

impl MembershipEntity for UserMembership {
    fn owner_pk(&self) -> Partition {
        self.pk.clone()
    }

    fn membership_pk(&self) -> &MembershipPartition {
        &self.membership_pk
    }

    fn set_membership_pk(&mut self, pk: MembershipPartition) {
        self.membership_pk = pk;
    }

    fn next_membership(&self) -> Option<&MembershipPartition> {
        self.next_membership.as_ref()
    }

    fn set_next_membership(&mut self, pk: Option<MembershipPartition>) {
        self.next_membership = pk;
    }

    fn expired_at(&self) -> i64 {
        self.expired_at
    }

    fn set_updated_at(&mut self, timestamp: i64) {
        self.updated_at = timestamp;
    }

    fn calculate_remaining_duration_days(&self) -> i32 {
        UserMembership::calculate_remaining_duration_days(self)
    }

    fn upsert_transact_write_item(&self) -> TransactWriteItem {
        self.upsert_transact_write_item()
    }
}
