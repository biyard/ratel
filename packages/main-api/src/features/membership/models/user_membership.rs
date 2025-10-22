use crate::{
    features::{membership::MembershipStatus, payment::PaymentMethod},
    types::*,
};
use bdk::prelude::*;

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

    // Status tracking with GSI for queries
    #[dynamo(index = "gsi2", pk, prefix = "STATUS", name = "find_by_status")]
    pub status: MembershipStatus,

    #[dynamo(index = "gsi2", sk)]
    pub expiration_timestamp: i64,

    // Credits management
    pub total_credits: i64,
    pub remaining_credits: i64,

    // Payment tracking
    pub transaction_id: Option<String>,
    pub payment_method: Option<PaymentMethod>,
    pub price_paid: i64,

    // Renewal tracking
    pub auto_renew: bool,
    pub renewal_count: i32,

    // Cancellation tracking
    pub cancelled_at: Option<i64>,
    pub cancellation_reason: Option<String>,
}

impl UserMembership {
    pub fn new(
        user_pk: Partition,
        membership_pk: Partition,
        duration_days: i32,
        credits: i64,
        price_paid: i64, // only dollers
    ) -> crate::Result<Self> {
        // Validation
        if !matches!(user_pk, Partition::User(_)) {
            return Err(crate::Error2::InvalidPartitionKey(
                "pk must be User partition".to_string(),
            ));
        }

        if !matches!(membership_pk, Partition::Membership(_)) {
            return Err(crate::Error2::InvalidPartitionKey(
                "membership_pk must be Membership partition".to_string(),
            ));
        }

        let created_at = chrono::Utc::now().timestamp_micros();

        // Fix: Convert to i64 before multiplication to prevent overflow
        let expired_at = if duration_days == 0 {
            // Lifetime membership (far future)
            i64::MAX
        } else {
            created_at + (duration_days as i64) * 24 * 60 * 60 * 1_000_000
        };

        Ok(Self {
            pk: user_pk,
            sk: EntityType::UserMembership,
            membership_pk,
            created_at,
            updated_at: created_at,
            expired_at,
            expiration_timestamp: expired_at,
            status: MembershipStatus::Active.into(),
            total_credits: credits,
            remaining_credits: credits,
            transaction_id: None,
            payment_method: None,
            price_paid,
            auto_renew: false,
            renewal_count: 0,
            cancelled_at: None,
            cancellation_reason: None,
        })
    }

    /// Check if membership is currently active and not expired
    pub fn is_active(&self) -> bool {
        self.status == MembershipStatus::Active
            && self.expired_at > chrono::Utc::now().timestamp_micros()
    }

    /// Check if membership is expired
    pub fn is_expired(&self) -> bool {
        self.expired_at <= chrono::Utc::now().timestamp_micros()
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

        let new_expiration = if duration_days == 0 {
            i64::MAX
        } else {
            base_time + (duration_days as i64) * 24 * 60 * 60 * 1_000_000
        };

        self.expired_at = new_expiration;
        self.expiration_timestamp = new_expiration;
        self.status = MembershipStatus::Active.into();
        self.renewal_count += 1;
        self.updated_at = now;

        Ok(())
    }

    /// Use credits from this membership
    pub fn use_credits(&mut self, amount: i64) -> crate::Result<()> {
        if self.remaining_credits < amount {
            return Err(crate::Error2::InsufficientCredits);
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
