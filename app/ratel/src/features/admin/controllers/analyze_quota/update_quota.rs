use crate::common::*;
use crate::common::models::auth::AdminUser;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::apps::apps::analyzes::AnalyzeQuotaConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(aide::OperationIo, schemars::JsonSchema))]
pub struct UpdateAnalyzeQuotaRequest {
    /// New limit for non-Enterprise members, applied space-wide.
    /// Must be `>= 0`. Setting to `0` effectively blocks every
    /// non-Enterprise create until raised.
    pub non_enterprise_limit: i64,
}

/// Upsert the singleton AnalyzeQuotaConfig row. First call creates it;
/// subsequent calls patch `non_enterprise_limit` + `updated_at`.
#[put("/api/admin/analyze-quota", _user: AdminUser)]
pub async fn update_analyze_quota(
    req: UpdateAnalyzeQuotaRequest,
) -> Result<AnalyzeQuotaConfig> {
    if req.non_enterprise_limit < 0 {
        return Err(Error::InvalidFormat);
    }

    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let (pk, sk) = AnalyzeQuotaConfig::keys();
    let now = get_now_timestamp_millis();

    let existing = AnalyzeQuotaConfig::get(cli, &pk, Some(sk.clone())).await?;
    let row = match existing {
        Some(_) => AnalyzeQuotaConfig::updater(&pk, &sk)
            .with_non_enterprise_limit(req.non_enterprise_limit)
            .with_updated_at(now)
            .execute(cli)
            .await
            .map_err(|e| {
                crate::error!("update_analyze_quota: {e}");
                Error::Internal
            })?,
        None => {
            let row = AnalyzeQuotaConfig {
                pk,
                sk,
                created_at: now,
                updated_at: now,
                non_enterprise_limit: req.non_enterprise_limit,
            };
            row.create(cli).await.map_err(|e| {
                crate::error!("update_analyze_quota create: {e}");
                Error::Internal
            })?;
            row
        }
    };

    Ok(row)
}
