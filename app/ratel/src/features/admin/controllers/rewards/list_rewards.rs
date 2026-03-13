use crate::common::models::auth::AdminUser;
use crate::features::admin::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RewardResponse {
    pub reward_behavior: RewardUserBehavior,
    pub point: i64,
    pub period: RewardPeriod,
    pub condition: RewardCondition,
}

impl From<Reward> for RewardResponse {
    fn from(value: Reward) -> Self {
        Self {
            reward_behavior: value.sk,
            point: value.point,
            period: value.period,
            condition: value.condition,
        }
    }
}

#[get("/api/admin/rewards", _user: AdminUser)]
pub async fn list_rewards() -> Result<ListResponse<RewardResponse>> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let opt = Reward::opt_all();
    let (items, _) = Reward::query(cli, Partition::Reward, opt).await?;

    let items = items.into_iter().map(RewardResponse::from).collect();

    Ok(ListResponse {
        items,
        bookmark: None,
    })
}
