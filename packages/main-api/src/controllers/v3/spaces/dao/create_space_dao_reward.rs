use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::{SpaceDao, SpaceDaoCandidate, SpaceDaoRewardUser};
use crate::types::{EntityType, Permissions, TeamGroupPermission};
use crate::utils::space_dao_reward::collect_space_dao_candidate_addresses;
use crate::{AppState, Error, transact_write_items};
use aide::NoApi;
use aws_sdk_dynamodb::types::TransactWriteItem;
use axum::Json;
use axum::extract::{Path, State};
use bdk::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct CreateSpaceDaoRewardRequest {
    pub reward_addresses: Vec<String>,
}

pub async fn create_space_dao_reward_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    Path(SpacePathParam { space_pk }): Path<SpacePathParam>,
    Json(req): Json<CreateSpaceDaoRewardRequest>,
) -> Result<Json<Vec<SpaceDaoRewardUser>>, Error> {
    permissions.permitted(TeamGroupPermission::SpaceEdit)?;

    if req.reward_addresses.is_empty() {
        return Err(Error::BadRequest("reward_addresses is empty".to_string()));
    }

    let dao = SpaceDao::get(&dynamo.client, space_pk.clone(), Some(EntityType::SpaceDao))
        .await?
        .ok_or(Error::DaoNotFound)?;

    let mut target_set = HashSet::new();
    for addr in req.reward_addresses {
        target_set.insert(addr.to_lowercase());
    }

    let candidates = collect_space_dao_candidate_addresses(
        &dynamo.client,
        &space_pk,
        dao.require_pre_survey,
        dao.require_post_survey,
    )
    .await?;
    let mut candidate_map: HashMap<String, SpaceDaoCandidate> = HashMap::new();
    for candidate in candidates {
        candidate_map.insert(candidate.evm_address.to_lowercase(), candidate);
    }

    let (existing, _) = SpaceDaoRewardUser::query(
        &dynamo.client,
        space_pk.clone(),
        SpaceDaoRewardUser::opt_all().sk("SPACE_DAO_REWARD#".to_string()),
    )
    .await
    .map_err(|err| {
        tracing::error!(
            "create_space_dao_reward: failed to load existing reward users: space={} err={:?}",
            space_pk,
            err
        );
        err
    })?;
    let mut existing_set = HashSet::new();
    for item in existing {
        existing_set.insert(item.evm_address.to_lowercase());
    }

    let mut pending: Vec<SpaceDaoRewardUser> = Vec::new();
    for addr in target_set {
        if existing_set.contains(&addr) {
            continue;
        }
        let Some(candidate) = candidate_map.get(&addr) else {
            continue;
        };
        let user_pk = candidate
            .user_pk
            .parse()
            .map_err(|_| Error::BadRequest("invalid user pk".to_string()))?;
        let item = SpaceDaoRewardUser::new(
            space_pk.clone(),
            user_pk,
            candidate.username.clone(),
            candidate.display_name.clone(),
            candidate.profile_url.clone(),
            candidate.evm_address.clone(),
        );
        pending.push(item);
    }

    let mut created = Vec::new();
    for chunk in pending.chunks(25) {
        let txs: Vec<TransactWriteItem> = chunk
            .iter()
            .map(|item| item.create_transact_write_item())
            .collect();
        transact_write_items!(&dynamo.client, txs)?;
        created.extend(chunk.iter().cloned());
    }

    if !created.is_empty() {
        let created_count = created.len() as i64;
        let remaining = dao.remaining_count + created_count;
        let total = dao.total_count + created_count;
        SpaceDao::updater(space_pk.clone(), EntityType::SpaceDao)
            .with_remaining_count(remaining)
            .with_total_count(total)
            .execute(&dynamo.client)
            .await?;
    }

    Ok(Json(created))
}
