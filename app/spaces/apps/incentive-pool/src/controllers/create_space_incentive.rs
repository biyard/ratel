use crate::models::SpaceIncentive;
use crate::*;
#[cfg(feature = "server")]
use common::SpaceUserRole;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSpaceIncentiveRequest {
    pub contract_address: String,
    pub deploy_block: i64,
}

#[post("/v3/spaces/{space_pk}/incentives", role: SpaceUserRole)]
pub async fn create_space_incentive(
    space_pk: SpacePartition,
    req: CreateSpaceIncentiveRequest,
) -> Result<SpaceIncentive> {
    if role != SpaceUserRole::Creator {
        return Err(Error::NoPermission);
    }

    let contract_address = req.contract_address.to_string();
    if contract_address.is_empty() {
        return Err(Error::BadRequest(
            "contract_address is required".to_string(),
        ));
    }

    if req.deploy_block < 0 {
        return Err(Error::BadRequest(
            "deploy_block must be 0 or greater".to_string(),
        ));
    }

    let common_config = common::CommonConfig::default();
    let dynamo = common_config.dynamodb();

    let incentive = SpaceIncentive::new(space_pk, contract_address, req.deploy_block);
    incentive.upsert(dynamo).await?;

    Ok(incentive)
}
