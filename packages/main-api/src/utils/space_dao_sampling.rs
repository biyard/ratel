use std::collections::HashSet;

use rand::prelude::IndexedRandom;

use crate::{transact_write_items, Result};
use crate::features::spaces::{SpaceDaoSampleUser, SpaceParticipant};
use crate::models::user::UserEvmAddress;
use crate::types::{EntityType, Partition};

pub async fn sample_space_dao_participants(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    sampling_count: i64,
) -> Result<Vec<SpaceDaoSampleUser>> {
    if sampling_count <= 0 {
        return Ok(Vec::new());
    }

    let (participants, _bookmark) =
        SpaceParticipant::find_by_space(cli, space_pk, SpaceParticipant::opt_all()).await?;

    let mut seen = HashSet::new();
    let mut unique: Vec<SpaceParticipant> = Vec::new();
    for participant in participants {
        let key = participant.user_pk.to_string();
        if seen.insert(key) {
            unique.push(participant);
        }
    }

    let evm_keys: Vec<_> = unique
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

    let mut eligible: Vec<(SpaceParticipant, String)> = Vec::new();
    for participant in unique {
        let key = participant.user_pk.to_string();
        if let Some(evm_address) = evm_map.get(&key) {
            eligible.push((participant, evm_address.clone()));
        }
    }

    let target = sampling_count as usize;
    let selected: Vec<(SpaceParticipant, String)> = {
        let mut rng = rand::rng();
        eligible
            .choose_multiple(&mut rng, target.min(eligible.len()))
            .cloned()
            .collect()
    };

    let mut results = Vec::with_capacity(selected.len());
    for (participant, evm_address) in selected {
        results.push(SpaceDaoSampleUser::new(
            space_pk.clone(),
            participant,
            evm_address,
        ));
    }

    if !results.is_empty() {
        let txs: Vec<_> = results
            .iter()
            .map(|item| item.create_transact_write_item())
            .collect();
        for chunk in txs.chunks(25) {
            transact_write_items!(cli, chunk.to_vec())?;
        }
    }

    Ok(results)
}
