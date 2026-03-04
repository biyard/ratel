use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteSpaceRewardRequest {
    pub sk: RewardKey,
}

#[delete("/api/spaces/{space_id}/rewards", role: SpaceUserRole)]
pub async fn delete_space_reward(
    space_id: SpacePartition,
    req: DeleteSpaceRewardRequest,
) -> Result<()> {
    use space_common::models::SpaceReward;
    SpaceReward::can_edit(&role)?;

    let common_config = common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_pk: Partition = space_id.into();

    let space_reward = SpaceReward::get(cli, &space_pk, Some(req.sk))
        .await?
        .ok_or(Error::from(SpaceRewardError::NotFound))?;

    // TODO: Add UserMembership credit refund when membership model is migrated
    SpaceReward::delete(cli, &space_reward.pk, Some(space_reward.sk)).await?;

    Ok(())
}
