use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RewardHistoryItem {
    pub created_at: i64,
    pub point: i64,
    pub space_title: Option<String>,
    pub action_name: Option<String>,
    pub transaction_id: Option<String>,
}

pub type ListRewardHistoryResponse = ListResponse<RewardHistoryItem>;

#[cfg(feature = "server")]
fn split_description(description: Option<String>) -> Option<String> {
    // Migration writes `description = "{space_pk}#{space_title}"`. Split on
    // the first `#` and take the tail as the human-readable title. Fall back
    // to the raw string when no separator exists (legacy rows written before
    // the migration that only carried the title).
    description.and_then(|d| {
        if let Some(idx) = d.find('#') {
            let title = &d[idx + 1..];
            if title.is_empty() {
                None
            } else {
                Some(title.to_string())
            }
        } else if d.is_empty() {
            None
        } else {
            Some(d)
        }
    })
}

#[get("/api/users/reward-history?username&bookmark")]
pub async fn list_reward_history_handler(
    username: String,
    bookmark: Option<String>,
) -> Result<ListRewardHistoryResponse> {
    use crate::common::models::reward::UserRewardHistory;
    use crate::common::types::CompositePartition;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let (users, _) = crate::features::auth::User::find_by_username(
        cli,
        &username,
        crate::features::auth::User::opt()
            .sk("TS#".to_string())
            .limit(1),
    )
    .await?;
    let user = users
        .into_iter()
        .find(|u| u.username == username)
        .ok_or(Error::NotFound(format!("User not found: {}", username)))?;

    // GSI1: pk = URH_BY_TARGET#{user_pk}##REWARD, sk = created_at desc.
    let pk_key = CompositePartition(user.pk.clone(), Partition::Reward);
    let gsi1_pk = UserRewardHistory::compose_gsi1_pk(pk_key);

    let opts = UserRewardHistory::opt_with_bookmark(bookmark)
        .limit(10)
        .scan_index_forward(false);

    let (items, next_bookmark) =
        UserRewardHistory::find_reward_by_user(cli, &gsi1_pk, opts)
            .await
            .map_err(|e| {
                crate::error!("failed to query UserRewardHistory by user: {e}");
                Error::NotFound("failed to list reward history".to_string())
            })?;

    let items: Vec<RewardHistoryItem> = items
        .into_iter()
        .map(|h| RewardHistoryItem {
            created_at: h.created_at,
            point: h.point,
            space_title: split_description(h.description),
            action_name: h.action_name,
            transaction_id: h.transaction_id,
        })
        .collect();

    Ok((items, next_bookmark).into())
}
