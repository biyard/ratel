use std::collections::{HashMap, HashSet};

use crate::features::spaces::SpaceDaoCandidate;
use crate::features::spaces::SpaceParticipant;
use crate::models::user::UserEvmAddress;
use crate::types::EntityType;
use crate::Result;
use bdk::prelude::*;

pub async fn collect_space_dao_candidate_addresses(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &crate::types::Partition,
) -> Result<Vec<SpaceDaoCandidate>> {
    let (participants, _bookmark) =
        SpaceParticipant::find_by_space(cli, space_pk, SpaceParticipant::opt_all()).await?;

    let mut seen_users = HashSet::new();
    let mut unique_users: Vec<SpaceParticipant> = Vec::new();
    for participant in participants {
        let key = participant.user_pk.to_string();
        if seen_users.insert(key) {
            unique_users.push(participant);
        }
    }

    let evm_keys: Vec<_> = unique_users
        .iter()
        .map(|p| (p.user_pk.clone(), EntityType::UserEvmAddress))
        .collect();

    let evm_addresses = if evm_keys.is_empty() {
        Vec::new()
    } else {
        UserEvmAddress::batch_get(cli, evm_keys).await?
    };

    let mut evm_map = std::collections::HashMap::new();
    for evm in evm_addresses {
        evm_map.insert(evm.pk.to_string(), evm.evm_address);
    }

    let mut seen_addresses = HashSet::new();
    let mut candidates = Vec::new();
    for participant in unique_users {
        let key = participant.user_pk.to_string();
        if let Some(evm_address) = evm_map.get(&key) {
            if seen_addresses.insert(evm_address.clone()) {
                candidates.push(SpaceDaoCandidate {
                    user_pk: key,
                    username: participant.username,
                    display_name: participant.display_name,
                    profile_url: participant.profile_url,
                    evm_address: evm_address.clone(),
                });
            }
        }
    }

    Ok(candidates)
}
