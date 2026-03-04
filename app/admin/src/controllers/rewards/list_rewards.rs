use crate::*;
use common::models::auth::AdminUser;

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

#[get("/api/admin/rewards?action", _user: AdminUser)]
pub async fn list_rewards(action: Option<String>) -> Result<ListResponse<RewardResponse>> {
    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let action: Option<RewardAction> = action
        .map(|s| s.parse())
        .transpose()
        .map_err(|_| Error::BadRequest("Invalid action".into()))?;

    let opt = Reward::opt_all();

    let (items, _) = if let Some(action) = action {
        let pk = Reward::compose_gsi1_pk(action);
        Reward::find_by_action(cli, &pk, opt).await?
    } else {
        Reward::query(cli, Partition::Reward, opt).await?
    };

    let items = items.into_iter().map(RewardResponse::from).collect();

    Ok(ListResponse {
        items,
        bookmark: None,
    })
}
