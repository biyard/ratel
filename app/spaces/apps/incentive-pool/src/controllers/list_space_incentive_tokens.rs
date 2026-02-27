#[cfg(feature = "server")]
use crate::models::SpaceIncentive;
use crate::models::SpaceIncentiveToken;
use crate::*;
#[cfg(feature = "server")]
use common::SpaceUserRole;

#[get(
    "/v3/spaces/{space_pk}/incentives/tokens?bookmark&limit",
    role: SpaceUserRole
)]
pub async fn list_space_incentive_tokens(
    space_pk: SpacePartition,
    bookmark: Option<String>,
    limit: Option<i32>,
) -> Result<ListResponse<SpaceIncentiveToken>> {
    let _ = role;

    let common_config = common::CommonConfig::default();
    let dynamo = common_config.dynamodb();

    let space_pk: Partition = space_pk.into();
    let incentive =
        SpaceIncentive::get(dynamo, &space_pk, Some(&EntityType::SpaceIncentive)).await?;

    let Some(incentive) = incentive else {
        return Ok((Vec::<SpaceIncentiveToken>::new(), None).into());
    };

    let mut opt = SpaceIncentiveToken::opt_with_bookmark(bookmark);
    if let Some(limit) = limit {
        opt = opt.limit(limit.max(1));
    }

    let (items, bookmark) =
        SpaceIncentiveToken::find_by_incentive_address(dynamo, &incentive.contract_address, opt)
            .await?;

    Ok((items, bookmark).into())
}
