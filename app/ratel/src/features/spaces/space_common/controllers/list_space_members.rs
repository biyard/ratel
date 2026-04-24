use crate::common::models::space::SpaceParticipant;
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpaceMemberResponse {
    pub user_id: UserPartition,
    pub display_name: String,
    pub username: String,
    pub profile_url: String,
}

#[cfg(feature = "server")]
impl From<SpaceParticipant> for SpaceMemberResponse {
    fn from(p: SpaceParticipant) -> Self {
        Self {
            user_id: p.user_pk.into(),
            display_name: p.display_name,
            username: p.username,
            profile_url: p.profile_url,
        }
    }
}

// Search tuning knobs. The GSI3 fast path is essentially free (DynamoDB
// handles `begins_with` natively on the sort key), so we only fall back to
// a page scan when that didn't already saturate the response. The scan
// caps bound the worst case for display_name prefix matching on huge
// spaces or non-ASCII queries that can't benefit from the GSI.
#[cfg(feature = "server")]
const SEARCH_PAGE_LIMIT: i32 = 50;
#[cfg(feature = "server")]
const SEARCH_MAX_PAGES: usize = 10;
#[cfg(feature = "server")]
const SEARCH_MAX_RESULTS: usize = 20;

#[get("/api/spaces/{space_id}/members?bookmark&query", role: SpaceUserRole)]
pub async fn list_space_members(
    space_id: SpacePartition,
    bookmark: Option<String>,
    query: Option<String>,
) -> Result<ListResponse<SpaceMemberResponse>> {
    let common_config = crate::common::CommonConfig::default();
    let dynamo = common_config.dynamodb();
    let space_pk: Partition = space_id.into();

    let normalized_query = query
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_lowercase);

    match normalized_query {
        None => {
            let opts = SpaceParticipant::opt_with_bookmark(bookmark).limit(50);
            let (participants, next_bookmark) =
                SpaceParticipant::find_by_space(dynamo, space_pk, opts).await?;

            let members = participants
                .into_iter()
                .map(SpaceMemberResponse::from)
                .collect();

            Ok(ListResponse {
                items: members,
                bookmark: next_bookmark,
            })
        }
        Some(q) => {
            let mut matches: Vec<SpaceMemberResponse> = Vec::new();
            let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

            // Fast path: DynamoDB GSI3 indexes participants by username, so a
            // `begins_with(username, q)` query reads only matching rows —
            // typically a single-digit RCU cost even on large spaces.
            let gsi_opts = SpaceParticipant::opt()
                .sk(q.clone())
                .limit(SEARCH_MAX_RESULTS as i32);
            // NOTE: SEARCH_MAX_RESULTS stays usize because it's compared
            // against Vec::len() in the loops below — the cast to i32 only
            // happens at the DDB boundary.
            let (gsi_matches, _) =
                SpaceParticipant::search_users_by_space(dynamo, space_pk.clone(), gsi_opts).await?;
            for p in gsi_matches {
                let pk_str = p.user_pk.to_string();
                if seen.insert(pk_str) {
                    matches.push(SpaceMemberResponse::from(p));
                    if matches.len() >= SEARCH_MAX_RESULTS {
                        break;
                    }
                }
            }

            // Fallback: scan by space for display_name prefix matches. Covers
            // cases where the visible display name diverges from the
            // underlying username — notably non-ASCII names whose username
            // is transliterated. Bounded by MAX_PAGES to keep worst-case
            // RCU predictable.
            if matches.len() < SEARCH_MAX_RESULTS {
                let mut next: Option<String> = None;
                let mut pages = 0;
                loop {
                    if pages >= SEARCH_MAX_PAGES || matches.len() >= SEARCH_MAX_RESULTS {
                        break;
                    }
                    let scan_opts =
                        SpaceParticipant::opt_with_bookmark(next.clone()).limit(SEARCH_PAGE_LIMIT);
                    let (participants, nb) =
                        SpaceParticipant::find_by_space(dynamo, space_pk.clone(), scan_opts)
                            .await?;

                    for p in participants {
                        let pk_str = p.user_pk.to_string();
                        if seen.contains(&pk_str) {
                            continue;
                        }
                        if p.display_name.to_lowercase().starts_with(&q) {
                            seen.insert(pk_str);
                            matches.push(SpaceMemberResponse::from(p));
                            if matches.len() >= SEARCH_MAX_RESULTS {
                                break;
                            }
                        }
                    }

                    pages += 1;
                    match nb {
                        Some(b) => next = Some(b),
                        None => break,
                    }
                }
            }

            Ok(ListResponse {
                items: matches,
                bookmark: None,
            })
        }
    }
}
