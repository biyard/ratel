use std::collections::{HashMap, HashSet};

use rand::prelude::IndexedRandom;

use crate::features::spaces::SpaceDaoCandidate;
use crate::features::spaces::SpaceParticipant;
use crate::features::spaces::polls::{
    Poll, PollQueryOption, PollUserAnswer, PollUserAnswerQueryOption,
};
use crate::models::user::UserEvmAddress;
use crate::types::{EntityType, Partition};
use crate::{Result, transact_write_items};
use bdk::prelude::*;

pub async fn collect_space_dao_candidate_addresses(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    require_pre_survey: bool,
    require_post_survey: bool,
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

    let required_users = if require_pre_survey || require_post_survey {
        load_required_user_pks(cli, space_pk, require_pre_survey, require_post_survey).await?
    } else {
        None
    };

    let mut seen_addresses = HashSet::new();
    let mut candidates = Vec::new();
    for participant in unique_users {
        let key = participant.user_pk.to_string();
        if let Some(ref required) = required_users {
            if !required.contains(&key) {
                continue;
            }
        }
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

async fn load_required_user_pks(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    require_pre_survey: bool,
    require_post_survey: bool,
) -> Result<Option<HashSet<String>>> {
    let mut required: Option<HashSet<String>> = None;

    if require_pre_survey {
        let Some(pre_poll) = default_poll_pk(space_pk) else {
            return Ok(Some(HashSet::new()));
        };
        let set = load_poll_user_pks(cli, space_pk, &pre_poll).await?;
        required = Some(set);
    }

    if require_post_survey {
        let post_polls = list_post_poll_pks(cli, space_pk).await?;
        if post_polls.is_empty() {
            return Ok(Some(HashSet::new()));
        }

        let mut post_required: Option<HashSet<String>> = None;
        for poll_pk in post_polls {
            let set = load_poll_user_pks(cli, space_pk, &poll_pk).await?;
            post_required = Some(match post_required {
                Some(prev) => prev
                    .intersection(&set)
                    .cloned()
                    .collect::<HashSet<String>>(),
                None => set,
            });
            if post_required.as_ref().map_or(false, |v| v.is_empty()) {
                break;
            }
        }

        let post_required = post_required.unwrap_or_default();
        required = Some(match required {
            Some(prev) => prev
                .intersection(&post_required)
                .cloned()
                .collect::<HashSet<String>>(),
            None => post_required,
        });
    }

    Ok(required)
}

fn default_poll_pk(space_pk: &Partition) -> Option<Partition> {
    match space_pk {
        Partition::Space(space_id) => Some(Partition::Poll(space_id.to_string())),
        _ => None,
    }
}

async fn list_post_poll_pks(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Vec<Partition>> {
    let (polls, _bookmark) = Poll::query(
        cli,
        space_pk,
        PollQueryOption::builder().sk("SPACE_POLL#".to_string()),
    )
    .await?;

    let mut items = Vec::new();
    for poll in polls {
        if poll.is_default_poll() {
            continue;
        }
        let poll_pk = match poll.sk {
            EntityType::SpacePoll(id) => Partition::Poll(id),
            _ => continue,
        };
        items.push((poll.created_at, poll_pk));
    }

    Ok(items.into_iter().map(|(_, pk)| pk).collect())
}

async fn load_poll_user_pks(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    poll_pk: &Partition,
) -> Result<HashSet<String>> {
    let mut users = HashSet::new();
    let mut bookmark: Option<String> = None;

    loop {
        let opt = if let Some(ref b) = bookmark {
            PollUserAnswerQueryOption::builder().bookmark(b.clone())
        } else {
            PollUserAnswerQueryOption::builder()
        };

        let (responses, next) = PollUserAnswer::find_by_space_pk(
            cli,
            &EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_pk.to_string()),
            opt,
        )
        .await?;

        for resp in responses {
            let user_pk = resp.user_pk.as_ref().map(|p| p.to_string()).or_else(|| {
                if let Partition::SpacePollUserAnswer(user) = &resp.pk {
                    Some(user.to_string())
                } else {
                    None
                }
            });

            if let Some(pk) = user_pk {
                users.insert(pk);
            }
        }

        if let Some(b) = next {
            bookmark = Some(b);
        } else {
            break;
        }
    }

    Ok(users)
}
