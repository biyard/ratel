use crate::{
    features::{membership::MembershipStatus, payment::PaymentMethod},
    types::*,
    *,
};

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

    // Fixed typo: membership_pk not memberhship_pk
    #[dynamo(prefix = "UM", name = "find_by_membership", index = "gsi1", pk)]
    pub membership_pk: Partition,
    pub status: MembershipStatus,

    // Credits management
    pub total_credits: i64,
    pub remaining_credits: i64,

    // Payment tracking
    pub purchase_id: Option<CompositePartition>,

    // Renewal tracking
    pub auto_renew: bool,

    // Cancellation tracking
    pub cancelled_at: Option<i64>,
    pub cancellation_reason: Option<String>,
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
            created_at + (duration_days as i64) * 24 * 60 * 60 * 1_000
        };

        Ok(Self {
            pk: user_pk.into(),
            sk: EntityType::UserMembership,
            membership_pk: membership_pk.into(),
            created_at,
            updated_at: created_at,
            expired_at,
            total_credits: credits,
            remaining_credits: credits,
            auto_renew: true,
            cancelled_at: None,
            cancellation_reason: None,
            purchase_id: None,
            status: MembershipStatus::Active,
        })
    }

    /// Check if membership is currently active and not expired
    pub fn is_active(&self) -> bool {
        self.status == MembershipStatus::Active
            && (self.expired_at == i64::MAX
                || self.expired_at > chrono::Utc::now().timestamp_micros())
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

    /// Cancel the membership
    pub fn cancel(&mut self, reason: Option<String>) {
        self.status = MembershipStatus::Cancelled;
        self.cancelled_at = Some(chrono::Utc::now().timestamp_micros());
        self.cancellation_reason = reason;
        self.updated_at = chrono::Utc::now().timestamp_micros();
        self.auto_renew = false;
    }

    /// Renew the membership
    pub fn renew(&mut self, duration_days: i32) -> crate::Result<()> {
        let now = chrono::Utc::now().timestamp_micros();

        // Extend from current expiration or now, whichever is later
        let base_time = if self.expired_at > now {
            self.expired_at
        } else {
            now
        };

        // Support -1 and 0 for infinite/lifetime memberships
        let new_expiration = if duration_days <= 0 {
            i64::MAX
        } else {
            base_time + (duration_days as i64) * 24 * 60 * 60 * 1_000_000
        };

        self.expired_at = new_expiration;
        self.status = MembershipStatus::Active.into();
        self.updated_at = now;

        Ok(())
    }

    /// Use credits from this membership
    pub fn use_credits(&mut self, amount: i64) -> crate::Result<()> {
        if self.remaining_credits < amount {
            return Err(crate::Error::InsufficientCredits);
        }

        self.remaining_credits -= amount;
        self.updated_at = chrono::Utc::now().timestamp_micros();

        Ok(())
    }

    /// Add credits to this membership
    pub fn add_credits(&mut self, amount: i64) {
        self.remaining_credits += amount;
        self.total_credits += amount;
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }

    /// Mark as expired if past expiration date
    pub fn check_and_update_expiration(&mut self) -> bool {
        if self.is_expired() && self.status == MembershipStatus::Active {
            self.status = MembershipStatus::Expired;
            self.updated_at = chrono::Utc::now().timestamp_micros();
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
        self.updated_at = chrono::Utc::now().timestamp_micros();
    }
}
