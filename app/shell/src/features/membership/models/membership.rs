use crate::features::membership::models::Currency;
use crate::features::membership::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum MembershipTier {
    #[default]
    Free,
    Pro,
    Max,
    Vip,
    Enterprise(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEnum, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "UPPERCASE")]
pub enum MembershipStatus {
    #[default]
    Active,
    Expired,
    Cancelled,
    Suspended,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Membership {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub credits: i64,
    pub tier: MembershipTier,
    pub price_dollars: i64,
    pub price_won: i64,
    pub display_order: i32,
    pub duration_days: i32,
    pub max_credits_per_space: i64,

    #[dynamo(index = "gsi1", pk, prefix = "ACTIVE", name = "find_active")]
    pub is_active: bool,
    #[dynamo(index = "gsi1", sk, prefix = "ORDER")]
    pub display_order_indexed: i32,
}

impl Membership {
    pub fn calculate_remaining_price(&self, currency: Currency, remaining_days: i32) -> i64 {
        if self.duration_days <= 0 || remaining_days <= 0 || remaining_days >= self.duration_days {
            return self.price_in_currency(currency);
        }

        let daily_price = self.price_in_currency(currency) as f64 / self.duration_days as f64;
        let remaining_price = daily_price * remaining_days as f64;
        remaining_price.round() as i64
    }

    pub fn price_in_currency(&self, currency: Currency) -> i64 {
        match currency {
            Currency::Usd => self.price_dollars,
            Currency::Krw => self.price_won,
        }
    }
}

#[cfg(feature = "server")]
impl Membership {
    pub async fn get_by_membership_tier(
        cli: &aws_sdk_dynamodb::Client,
        tier: &MembershipTier,
    ) -> Result<Self> {
        let pk = Partition::Membership(tier.to_string());
        let m = Membership::get(cli, pk, Some(EntityType::Membership))
            .await?
            .ok_or_else(|| Error::NotFound("Membership not found".to_string()))?;
        Ok(m)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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

    pub total_credits: i64,
    pub remaining_credits: i64,

    pub auto_renew: bool,
    pub next_membership: Option<MembershipPartition>,
}

impl UserMembership {
    pub fn new(
        user_pk: UserPartition,
        membership_pk: MembershipPartition,
        duration_days: i32,
        credits: i64,
    ) -> Result<Self> {
        let created_at = common::utils::time::now();
        let expired_at = if duration_days <= 0 {
            i64::MAX
        } else {
            created_at + (duration_days as i64) * (24 * 60 * 60 * 1_000)
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

    pub fn is_infinite(&self) -> bool {
        self.expired_at == i64::MAX
    }

    pub fn day_unit(&self) -> i64 {
        24 * 60 * 60 * 1_000
    }

    pub fn calculate_remaining_duration_days(&self) -> i32 {
        if self.is_infinite() {
            return -1;
        }

        let now = common::utils::time::now();
        if self.expired_at <= now {
            return 0;
        }

        let remaining_millis = self.expired_at - now;
        let remaining_days = remaining_millis / self.day_unit();
        remaining_days as i32
    }

    pub fn with_purchase_id(self, _purchase_id: CompositePartition) -> Self {
        self
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MembershipResponse {
    pub id: MembershipPartition,
    pub tier: MembershipTier,
    pub price_dollars: i64,
    pub credits: i64,
    pub duration_days: i32,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub max_credits_per_space: i64,
}

impl From<Membership> for MembershipResponse {
    fn from(membership: Membership) -> Self {
        Self {
            id: membership.pk.into(),
            tier: membership.tier,
            price_dollars: membership.price_dollars,
            credits: membership.credits,
            duration_days: membership.duration_days,
            display_order: membership.display_order,
            is_active: membership.is_active,
            created_at: membership.created_at,
            updated_at: membership.updated_at,
            max_credits_per_space: membership.max_credits_per_space,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
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
