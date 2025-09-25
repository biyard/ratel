use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;
use serde_json;
use std::collections::HashMap;

/*
Membership Level Specifications:

Reward Boosters Available: 0x (no boost), 1x, 2x, 10x, 100x, 1000x (customizable for enterprise)

Free:
- Can create posts and spaces unlimited (no quotas tracked)

Pro:
- Includes all Free tier
- 1x booster reward space -> 20
- 2x booster reward space -> 10

Max:
- Includes all Pro tier
- 1x booster reward space -> +30 (totally 50)
- 2x booster reward space -> +10 (totally 20)
- 10x booster reward space -> 10

VIP:
- Includes all Max tier
- 1x booster reward space -> +50 (totally 100)
- 2x booster reward space -> +10 (totally 30)
- 10x booster reward space -> +10 (totally 20)
- 100x booster reward space -> 10

Enterprise (fully customizable):
- Example configuration:
  - 1x booster reward space -> 1000
  - 2x booster reward space -> 285
  - 10x booster reward space -> 200
  - 100x booster reward space -> 150
  - 1000x booster reward space -> 4
*/

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserMembership {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    /// The membership type (Free, Pro, Max, VIP, Enterprise, Admin)
    pub membership_type: Membership,

    /// Start timestamp in microseconds
    pub subscription_start: i64,

    /// End timestamp in microseconds  
    pub subscription_end: i64,

    /// HashMap where key is booster type (0=no boost, 1=1x, 2=2x, 10=10x, 100=100x, 1000=1000x)
    /// and value is remaining quota for the month - stored as JSON string
    pub space_capabilities: String,
}

/// Builder for UserMembership with fluent API
#[derive(Debug, Clone)]
pub struct UserMembershipBuilder {
    user_id: String,
    membership_type: Membership,
    custom_capabilities: Option<HashMap<u32, i32>>,
    subscription_duration_days: Option<i64>,
}

impl UserMembershipBuilder {
    /// Create a new UserMembership builder for the given user_id
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            membership_type: Membership::Free,
            custom_capabilities: None,
            subscription_duration_days: None,
        }
    }

    /// Set membership to Free tier
    pub fn with_free(mut self) -> Self {
        self.membership_type = Membership::Free;
        self
    }

    /// Set membership to Pro tier
    pub fn with_pro(mut self) -> Self {
        self.membership_type = Membership::Pro;
        self
    }

    /// Set membership to Max tier
    pub fn with_max(mut self) -> Self {
        self.membership_type = Membership::Max;
        self
    }

    /// Set membership to VIP tier
    pub fn with_vip(mut self) -> Self {
        self.membership_type = Membership::VIP;
        self
    }

    /// Set membership to Enterprise tier with optional custom capabilities
    pub fn with_enterprise(mut self, custom_capabilities: Option<HashMap<u32, i32>>) -> Self {
        self.membership_type = Membership::Enterprise;
        self.custom_capabilities = custom_capabilities;
        self
    }

    /// Set membership to Admin tier
    pub fn with_admin(mut self) -> Self {
        self.membership_type = Membership::Admin;
        self
    }

    /// Set custom subscription duration in days (overrides defaults)
    pub fn with_duration_days(mut self, days: i64) -> Self {
        self.subscription_duration_days = Some(days);
        self
    }

    /// Build the UserMembership instance
    pub fn build(self) -> UserMembership {
        let pk = Partition::User(self.user_id.clone());
        let sk = EntityType::UserMembership;
        let now = get_now_timestamp_millis();
        let now_micros = chrono::Utc::now().timestamp_micros();

        // Determine subscription duration
        let default_duration_days = match self.membership_type {
            Membership::Enterprise | Membership::Admin => 365, // 1 year
            _ => 30,                                           // 30 days for other tiers
        };
        let duration_days = self
            .subscription_duration_days
            .unwrap_or(default_duration_days);
        let subscription_end = now_micros + (duration_days * 24 * 60 * 60 * 1_000_000);

        // Create capabilities based on membership type
        let capabilities = self.create_capabilities();
        let capabilities_json = serde_json::to_string(&capabilities).unwrap_or_default();

        UserMembership {
            pk: pk.into(),
            sk,
            created_at: now,
            updated_at: now,
            membership_type: self.membership_type,
            subscription_start: now_micros,
            subscription_end,
            space_capabilities: capabilities_json,
        }
    }

    fn create_capabilities(&self) -> HashMap<u32, i32> {
        match self.membership_type {
            Membership::Free => {
                HashMap::new()
            }
            Membership::Pro => {
                let mut caps = HashMap::new();
                caps.insert(1, 20);
                caps.insert(2, 10);
                caps
            }
            Membership::Max => {
                let mut caps = HashMap::new();
                caps.insert(1, 50);
                caps.insert(2, 20);
                caps.insert(10, 10);
                caps
            }
            Membership::VIP => {
                let mut caps = HashMap::new();
                caps.insert(1, 100);
                caps.insert(2, 30);
                caps.insert(10, 20);
                caps.insert(100, 10);
                caps
            }
            Membership::Enterprise => {
                if let Some(custom_caps) = &self.custom_capabilities {
                    custom_caps.clone()
                } else {
                    let mut caps = HashMap::new();
                    caps.insert(1, 100);
                    caps.insert(2, 100);
                    caps.insert(10, 100);
                    caps.insert(100, 100);
                    caps
                }
            }
            Membership::Admin => {
                let mut caps = HashMap::new();
                caps.insert(1, -1);
                caps.insert(2, -1);
                caps.insert(10, -1);
                caps.insert(100, -1);
                caps.insert(1000, -1);
                caps
            }
        }
    }
}

impl UserMembership {
    pub fn builder(user_id: String) -> UserMembershipBuilder {
        UserMembershipBuilder::new(user_id)
    }

    pub fn get_space_capabilities(&self) -> HashMap<u32, i32> {
        serde_json::from_str(&self.space_capabilities).unwrap_or_default()
    }

    pub fn set_space_capabilities(&mut self, capabilities: HashMap<u32, i32>) {
        self.space_capabilities = serde_json::to_string(&capabilities).unwrap_or_default();
    }

    /// Create UserMembership from Membership type with default capabilities
    pub fn from_membership(user_id: String, membership: Membership) -> Self {
        match membership {
            Membership::Free => Self::builder(user_id).with_free().build(),
            Membership::Pro => Self::builder(user_id).with_pro().build(),
            Membership::Max => Self::builder(user_id).with_max().build(),
            Membership::VIP => Self::builder(user_id).with_vip().build(),
            Membership::Enterprise => Self::builder(user_id).with_enterprise(None).build(),
            Membership::Admin => Self::builder(user_id).with_admin().build(),
        }
    }

    /// Check if the membership is currently active
    pub fn is_active(&self) -> bool {
        let now = chrono::Utc::now().timestamp_micros();
        now >= self.subscription_start && now <= self.subscription_end
    }

    /// Check if user can create a space with the given booster type
    pub fn can_create_space(&self, booster_type: u32) -> bool {
        if self.membership_type == Membership::Admin {
            return true; // Admin has unlimited quota
        }

        if booster_type == 0 {
            return true; // No-boost spaces are unlimited for all tiers
        }
        if self.membership_type == Membership::Free {
            return false; // Free tier cannot create boosted spaces
        }

        let capabilities = self.get_space_capabilities();
        capabilities
            .get(&booster_type)
            .map(|&count| count > 0 || count == -1) // -1 means unlimited
            .unwrap_or(false)
    }

    /// Consume one space creation quota for the given booster type
    pub fn consume_space_quota(&mut self, booster_type: u32) -> bool {
        if self.membership_type == Membership::Free && booster_type == 0 {
            return true; // Free tier has unlimited basic spaces
        }

        let mut capabilities = self.get_space_capabilities();
        if let Some(count) = capabilities.get_mut(&booster_type) {
            if *count == -1 {
                return true; // Unlimited
            }
            if *count > 0 {
                *count -= 1;
                self.set_space_capabilities(capabilities);
                return true;
            }
        }
        false
    }

    /// Reset monthly quotas (should be called at the beginning of each billing cycle)
    pub fn reset_monthly_quotas(&mut self) {
        let mut capabilities = HashMap::new();
        match self.membership_type {
            Membership::Free => {
                // Free tier has unlimited basic spaces, no quotas to reset
            }
            Membership::Pro => {
                capabilities.insert(1, 20); // 1x booster -> 20 spaces
                capabilities.insert(2, 10); // 2x booster -> 10 spaces
            }
            Membership::Max => {
                // Included all Pro tier + additional quotas
                capabilities.insert(1, 50); // 1x booster -> 50 spaces total
                capabilities.insert(2, 20); // 2x booster -> 20 spaces total
                capabilities.insert(10, 10); // 10x booster -> 10 spaces
            }
            Membership::VIP => {
                // Included all Max tier + additional quotas
                capabilities.insert(1, 100); // 1x booster -> 100 spaces total
                capabilities.insert(2, 30); // 2x booster -> 30 spaces total
                capabilities.insert(10, 20); // 10x booster -> 20 spaces total
                capabilities.insert(100, 10); // 100x booster -> 10 spaces
            }
            Membership::Enterprise | Membership::Admin => {
                // Enterprise uses custom capabilities, Admin has unlimited access
                if self.membership_type == Membership::Admin {
                    capabilities.insert(1, -1);
                    capabilities.insert(2, -1);
                    capabilities.insert(10, -1);
                    capabilities.insert(100, -1);
                    capabilities.insert(1000, -1);
                } else {
                    // For Enterprise, we should preserve existing custom capabilities or use defaults
                    let current_caps = self.get_space_capabilities();
                    if !current_caps.is_empty() {
                        capabilities = current_caps; // Keep existing custom capabilities
                    } else {
                        // Use example Enterprise defaults
                        capabilities.insert(1, 100); // 1x booster -> 1000 spaces
                        capabilities.insert(2, 100); // 2x booster -> 285 spaces
                        capabilities.insert(10, 100); // 10x booster -> 200 spaces
                        capabilities.insert(100, 100); // 100x booster -> 150 spaces
                        capabilities.insert(1000, 100); // 1000x booster -> 4 spaces
                    }
                }
            }
        }
        self.set_space_capabilities(capabilities);
    }
}
