use crate::common::*;

/// Singleton sidecar per user, tracking onboarding interstitials the user
/// has dismissed. We deliberately keep this **off** the main `User`
/// entity so feature-flag isolation (cross-posting code can be removed
/// without touching `User`) and future onboarding flags can be added
/// without bloating the core entity.
///
/// Default when the row is absent is `false` for every flag (= never seen).
///
/// Design doc: docs/superpowers/specs/2026-04-28-cross-posting-design.md
/// (`UserOnboardingFlags` section). FR-2 #13.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserOnboardingFlags {
    #[dynamo(prefix = "UOF", pk)]
    pub pk: Partition, // User(user_id)

    pub sk: EntityType, // UserOnboardingFlags (singleton)

    /// Set true on Continue / Skip from the post-signup interstitial.
    pub cross_posting_interstitial_seen: bool,

    pub created_at: i64,
    pub updated_at: i64,
}
