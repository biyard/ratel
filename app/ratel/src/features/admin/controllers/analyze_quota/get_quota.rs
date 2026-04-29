use crate::common::*;
use crate::common::models::auth::AdminUser;
use crate::features::spaces::pages::apps::apps::analyzes::AnalyzeQuotaConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct AnalyzeQuotaResponse {
    /// Limit currently enforced for non-Enterprise members. When the
    /// underlying row hasn't been initialised yet, this echoes the
    /// hardcoded fallback (`AnalyzeQuotaConfig::DEFAULT_LIMIT`).
    pub non_enterprise_limit: i64,
    /// `true` when the row exists in DDB; `false` means the response
    /// is the fallback default and a `PUT` will create the row.
    pub exists: bool,
}

#[get("/api/admin/analyze-quota", _user: AdminUser)]
pub async fn get_analyze_quota() -> Result<AnalyzeQuotaResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let (pk, sk) = AnalyzeQuotaConfig::keys();
    let row = AnalyzeQuotaConfig::get(cli, &pk, Some(sk)).await?;
    Ok(match row {
        Some(r) => AnalyzeQuotaResponse {
            non_enterprise_limit: r.non_enterprise_limit,
            exists: true,
        },
        None => AnalyzeQuotaResponse {
            non_enterprise_limit: AnalyzeQuotaConfig::DEFAULT_LIMIT,
            exists: false,
        },
    })
}
