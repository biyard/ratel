use crate::features::spaces::pages::apps::apps::analyzes::*;

/// Singleton row controlling the analyze-report creation quota for
/// non-enterprise tiers. A single deployment-wide value — bumped or
/// lowered through the admin API. Enterprise tier always bypasses
/// the gate, so this only governs Free / Pro / Max / Vip members.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, schemars::JsonSchema, aide::OperationIo)
)]
pub struct AnalyzeQuotaConfig {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    /// Maximum analyze reports a non-enterprise user may have in a
    /// single space. Enterprise membership is unlimited.
    #[serde(default)]
    pub non_enterprise_limit: i64,
}

#[cfg(feature = "server")]
impl AnalyzeQuotaConfig {
    /// Hardcoded fallback used when the row hasn't been initialised
    /// yet. Lets the gate work out of the box on a fresh deployment;
    /// admin can override via `PUT /api/admin/analyze-quota` whenever.
    pub const DEFAULT_LIMIT: i64 = 2;

    pub fn keys() -> (Partition, EntityType) {
        (Partition::AnalyzeQuotaConfig, EntityType::AnalyzeQuotaConfig)
    }

    /// Loads the configured limit, falling back to `DEFAULT_LIMIT`
    /// when no row exists yet. Errors propagate so callers can decide
    /// whether to fail closed or open — `create_analyze_report`
    /// fails closed (rejects the create) on any read error.
    pub async fn get_limit(cli: &aws_sdk_dynamodb::Client) -> crate::common::Result<i64> {
        let (pk, sk) = Self::keys();
        let row = AnalyzeQuotaConfig::get(cli, &pk, Some(sk)).await?;
        Ok(row
            .map(|r| r.non_enterprise_limit)
            .unwrap_or(Self::DEFAULT_LIMIT))
    }
}
