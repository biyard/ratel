use crate::features::spaces::apps::rewards::*;
#[cfg(feature = "server")]
use crate::common::models::auth::OptionalUser;
use crate::features::spaces::space_common::models::SpaceRewardResponse;

#[get("/api/spaces/{space_id}/rewards?action_key", user: OptionalUser)]
pub async fn list_space_rewards(
    space_id: SpacePartition,
    action_key: Option<EntityType>,
) -> Result<ListResponse<SpaceRewardResponse>> {
    use crate::common::models::reward::UserReward;
    use crate::features::spaces::space_common::models::SpaceReward;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_rewards = SpaceReward::list_by_action(cli, space_id.clone(), action_key).await?;

    let user_rewards = if let Some(user) = user.0 {
        let user_reward_keys: Vec<_> = space_rewards
            .iter()
            .filter_map(|reward| UserReward::keys(user.pk.clone(), reward.sk.clone()).ok())
            .collect();
        UserReward::batch_get(cli, user_reward_keys)
            .await
            .unwrap_or_default()
    } else {
        vec![]
    };

    Ok(ListResponse {
        items: space_rewards
            .into_iter()
            .map(|reward| {
                if let Some(user_reward) = user_rewards.iter().find(|ur| ur.sk == reward.sk) {
                    SpaceRewardResponse::from((reward, user_reward.clone()))
                } else {
                    SpaceRewardResponse::from(reward)
                }
            })
            .collect(),
        bookmark: None,
    })
}
