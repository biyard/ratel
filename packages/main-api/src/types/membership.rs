#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
)]
#[repr(u8)]
pub enum Membership {
    #[default]
    Free = 1,
    Pro = 2,
    Max = 3,
    VIP = 4,
    Enterprise = 5,
    Admin = 99,
}

use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct MembershipInfo {
    pub membership_type: Membership,
    pub subscription_start: i64, // timestamp in microseconds
    pub subscription_end: i64,   // timestamp in microseconds
    /// HashMap where key is booster type (0=no boost, 2=2x, 10=10x, 100=100x, 1000=1000x) and value is remaining quota for the month
    pub space_capabilities: HashMap<u32, i32>,
}

impl MembershipInfo {
    pub fn new_free() -> Self {
        Self {
            membership_type: Membership::Free,
            subscription_start: chrono::Utc::now().timestamp_micros(),
            subscription_end: chrono::Utc::now().timestamp_micros()
                + (30 * 24 * 60 * 60 * 1_000_000), // 30 days
            space_capabilities: HashMap::new(), // Free tier has unlimited basic spaces
        }
    }

    pub fn new_pro() -> Self {
        let mut capabilities = HashMap::new();
        capabilities.insert(2, 20); // 2x booster -> 20 spaces
        capabilities.insert(10, 10); // 10x booster -> 10 spaces

        Self {
            membership_type: Membership::Pro,
            subscription_start: chrono::Utc::now().timestamp_micros(),
            subscription_end: chrono::Utc::now().timestamp_micros()
                + (30 * 24 * 60 * 60 * 1_000_000),
            space_capabilities: capabilities,
        }
    }

    pub fn new_max() -> Self {
        let mut capabilities = HashMap::new();
        capabilities.insert(2, 50); // 2x booster -> 50 spaces (20+30)
        capabilities.insert(10, 20); // 10x booster -> 20 spaces (10+10)
        capabilities.insert(100, 10); // 100x booster -> 10 spaces

        Self {
            membership_type: Membership::Max,
            subscription_start: chrono::Utc::now().timestamp_micros(),
            subscription_end: chrono::Utc::now().timestamp_micros()
                + (30 * 24 * 60 * 60 * 1_000_000),
            space_capabilities: capabilities,
        }
    }

    pub fn new_vip() -> Self {
        let mut capabilities = HashMap::new();
        capabilities.insert(2, 100); // 2x booster -> 100 spaces (50+50)
        capabilities.insert(10, 30); // 10x booster -> 30 spaces (20+10)
        capabilities.insert(100, 20); // 100x booster -> 20 spaces (10+10)
        capabilities.insert(1000, 10); // 1000x booster -> 10 spaces

        Self {
            membership_type: Membership::VIP,
            subscription_start: chrono::Utc::now().timestamp_micros(),
            subscription_end: chrono::Utc::now().timestamp_micros()
                + (30 * 24 * 60 * 60 * 1_000_000),
            space_capabilities: capabilities,
        }
    }

    pub fn new_enterprise(custom_capabilities: HashMap<u32, i32>) -> Self {
        Self {
            membership_type: Membership::Enterprise,
            subscription_start: chrono::Utc::now().timestamp_micros(),
            subscription_end: chrono::Utc::now().timestamp_micros()
                + (30 * 24 * 60 * 60 * 1_000_000),
            space_capabilities: custom_capabilities,
        }
    }

    /// Check if the membership is currently active
    pub fn is_active(&self) -> bool {
        let now = chrono::Utc::now().timestamp_micros();
        now >= self.subscription_start && now <= self.subscription_end
    }

    /// Check if user can create a space with the given booster type
    pub fn can_create_space(&self, booster_type: u32) -> bool {
        if self.membership_type == Membership::Free {
            return booster_type == 0; // Free tier can only create no-boost spaces (unlimited)
        }

        self.space_capabilities
            .get(&booster_type)
            .map(|&count| count > 0)
            .unwrap_or(false)
    }

    /// Consume one space creation quota for the given booster type
    pub fn consume_space_quota(&mut self, booster_type: u32) -> bool {
        if self.membership_type == Membership::Free && booster_type == 0 {
            return true; // Free tier has unlimited basic spaces
        }

        if let Some(count) = self.space_capabilities.get_mut(&booster_type) {
            if *count > 0 {
                *count -= 1;
                return true;
            }
        }
        false
    }

    /// Reset monthly quotas (should be called at the beginning of each billing cycle)
    pub fn reset_monthly_quotas(&mut self) {
        match self.membership_type {
            Membership::Free => {
                self.space_capabilities.clear();
            }
            Membership::Pro => {
                self.space_capabilities.insert(2, 20);
                self.space_capabilities.insert(10, 10);
            }
            Membership::Max => {
                self.space_capabilities.insert(2, 50);
                self.space_capabilities.insert(10, 20);
                self.space_capabilities.insert(100, 10);
            }
            Membership::VIP => {
                self.space_capabilities.insert(2, 100);
                self.space_capabilities.insert(10, 30);
                self.space_capabilities.insert(100, 20);
                self.space_capabilities.insert(1000, 10);
            }
            Membership::Enterprise => {
                // Enterprise quotas are custom and should be set manually
            }
            Membership::Admin => {
                // Admins have unlimited access, no quotas
            }
        }
    }
}
