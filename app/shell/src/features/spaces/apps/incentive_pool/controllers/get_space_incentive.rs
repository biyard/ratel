use crate::features::spaces::apps::incentive_pool::models::SpaceIncentive;
use crate::features::spaces::apps::incentive_pool::*;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;

#[get("/api/spaces/{space_pk}/incentives", role: SpaceUserRole)]
pub async fn get_space_incentive(space_pk: SpacePartition) -> Result<SpaceIncentive> {
    SpaceIncentive::can_view(role)?;
    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();

    let space_pk: Partition = space_pk.into();
    let incentive = SpaceIncentive::get(dynamo, &space_pk, Some(&EntityType::SpaceIncentive))
        .await?
        .ok_or_else(|| Error::NotFound("Space incentive not found".to_string()))?;

    Ok(incentive)
}
