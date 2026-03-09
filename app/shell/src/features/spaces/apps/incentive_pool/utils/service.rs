use crate::features::spaces::apps::incentive_pool::interop::{deploy_space_incentive, DeploySpaceIncentiveRequest};
use crate::features::spaces::apps::incentive_pool::models::{SpaceIncentive, SpaceIncentiveToken};
use crate::features::spaces::apps::incentive_pool::*;

const TOKEN_PAGE_LIMIT: i32 = 100;

pub(crate) async fn register_incentive_pool(
    space_id: SpacePartition,
    contract_mode: i64,
    ranking_bps: i64,
    incentive_recipient_count: i64,
) -> Result<(SpaceIncentive, Vec<SpaceIncentiveToken>)> {
    let deploy_req = DeploySpaceIncentiveRequest {
        admins: vec![],
        incentive_recipient_count,
        ranking_bps,
        mode: contract_mode,
        env: option_env!("ENV").unwrap_or("local").to_string(),
    };
    let deploy = deploy_space_incentive(deploy_req).await?;
    let created = create_space_incentive(
        space_id.clone(),
        CreateSpaceIncentiveRequest {
            contract_address: deploy.incentive_address,
            deploy_block: deploy.deploy_block,
        },
    )
    .await?;
    let loaded_tokens = load_tokens(space_id).await?;

    Ok((created, loaded_tokens))
}

pub(crate) async fn refresh_tokens(space_id: SpacePartition) -> Result<Vec<SpaceIncentiveToken>> {
    refresh_space_incentive_tokens(space_id.clone()).await?;
    load_tokens(space_id).await
}

pub(crate) async fn load_incentive_and_tokens(
    space_id: SpacePartition,
) -> Result<(Option<SpaceIncentive>, Vec<SpaceIncentiveToken>)> {
    let incentive = match get_space_incentive(space_id.clone()).await {
        Ok(incentive) => Some(incentive),
        Err(Error::NotFound(_)) => None,
        Err(err) => return Err(err),
    };
    let tokens = load_tokens(space_id).await?;

    Ok((incentive, tokens))
}

pub(crate) async fn load_tokens(space_id: SpacePartition) -> Result<Vec<SpaceIncentiveToken>> {
    let mut bookmark = None;
    let mut items = Vec::new();

    loop {
        let response =
            list_space_incentive_tokens(space_id.clone(), bookmark.clone(), Some(TOKEN_PAGE_LIMIT))
                .await?;
        items.extend(response.items);

        match response.bookmark {
            Some(next) if !next.is_empty() => bookmark = Some(next),
            _ => break,
        }
    }

    Ok(items)
}
