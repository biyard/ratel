use crate::*;
use space_common::models::SpaceRewardResponse;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSpaceRewardRequest {
    pub sk: RewardKey,
    #[serde(default)]
    pub description: String,
    pub credits: i64,
}

#[put("/api/spaces/{space_id}/rewards", role: SpaceUserRole)]
pub async fn update_space_reward(
    space_id: SpacePartition,
    req: UpdateSpaceRewardRequest,
) -> Result<SpaceRewardResponse> {
    use common::utils::time::get_now_timestamp_millis;
    use space_common::models::SpaceReward;

    if req.credits < 1 {
        return Err(Error::BadRequest("Credits must be at least 1".into()));
    }

    SpaceReward::can_edit(&role)?;

    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_pk: Partition = space_id.into();

    let mut space_reward = SpaceReward::get(cli, &space_pk, Some(req.sk))
        .await?
        .ok_or(Error::from(SpaceRewardError::NotFound))?;

    let now = get_now_timestamp_millis();

    // TODO: Add UserMembership credit adjustment when membership model is migrated
    SpaceReward::updater(&space_reward.pk, &space_reward.sk)
        .with_description(req.description.clone())
        .with_credits(req.credits)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    space_reward.updated_at = now;
    space_reward.description = req.description;
    space_reward.credits = req.credits;

    Ok(space_reward.into())
}
