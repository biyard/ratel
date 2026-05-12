use crate::common::models::auth::AdminUser;
use crate::common::services::pending_reward_retry::RetryStats;
use crate::features::admin::*;

#[post("/api/admin/internal/run-pending-reward-retry", _user: AdminUser)]
pub async fn run_pending_reward_retry() -> Result<RetryStats> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    crate::common::services::pending_reward_retry::retry_pending_rewards(cli).await
}
