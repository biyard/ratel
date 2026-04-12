use crate::features::spaces::pages::apps::apps::incentive_pool::models::SpaceIncentive;
use crate::features::spaces::pages::apps::apps::incentive_pool::*;
use crate::features::spaces::pages::apps::types::SpaceAppError;
#[cfg(feature = "server")]
use crate::common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSpaceIncentiveRequest {
    pub contract_address: String,
    pub deploy_block: i64,
}

#[post("/api/spaces/{space_pk}/incentives", role: SpaceUserRole)]
pub async fn create_space_incentive(
    space_pk: SpacePartition,
    req: CreateSpaceIncentiveRequest,
) -> Result<SpaceIncentive> {
    SpaceIncentive::can_edit(role)?;

    if req.contract_address.is_empty() {
        return Err(SpaceAppError::IncentiveAddressRequired.into());
    }

    if req.deploy_block < 0 {
        return Err(SpaceAppError::IncentiveChainRequired.into());
    }

    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();

    let incentive = SpaceIncentive::new(space_pk, req.contract_address, req.deploy_block);
    incentive.upsert(dynamo).await?;

    Ok(incentive)
}
