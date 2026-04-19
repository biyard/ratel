use super::super::{dto::RewardsResponse, *};

#[cfg(feature = "server")]
async fn sum_local_points(
    cli: &crate::common::aws_sdk_dynamodb::Client,
    user_pk: &Partition,
) -> i64 {
    use crate::common::models::reward::UserReward;

    // UserReward lives at pk = CompositePartition(user_pk, Reward). Paginate
    // through the entire set and sum total_points — this is the authoritative
    // local record of everything the user has ever earned.
    let pk_key =
        crate::common::types::CompositePartition(user_pk.clone(), Partition::Reward);

    let mut total: i64 = 0;
    let mut bookmark: Option<String> = None;
    let mut guard = 0u32;
    loop {
        let opts = UserReward::opt_with_bookmark(bookmark.clone()).limit(100);
        match UserReward::query(cli, pk_key.clone(), opts).await {
            Ok((items, next)) => {
                for item in &items {
                    total = total.saturating_add(item.total_points);
                }
                bookmark = next;
                if bookmark.is_none() {
                    break;
                }
            }
            Err(e) => {
                crate::error!("failed to query local UserReward for {user_pk}: {e}");
                break;
            }
        }
        guard += 1;
        if guard > 50 {
            tracing::warn!(user_pk = %user_pk, "sum_local_points: hit page guard");
            break;
        }
    }
    total
}

#[cfg(feature = "server")]
async fn fetch_rewards(user_pk: Partition, month: Option<String>) -> Result<RewardsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let biyard = cfg.biyard();
    let cli = cfg.dynamodb();

    let month = month.unwrap_or_else(|| utils::time::current_month());

    // Local total is the authoritative source for "how many points does this
    // user have on Ratel right now". Biyard is queried for the project-wide
    // pool info and the token contract; if Biyard is offline or returns less
    // than the local total (e.g. a pending reward has not yet replayed), we
    // still surface what the user actually earned.
    let local_points = sum_local_points(cli, &user_pk).await;

    let biyard_balance = match biyard.get_user_balance(user_pk.clone(), month.clone()).await {
        Ok(b) => Some(b),
        Err(e) => {
            crate::error!(
                "Biyard get_user_balance failed for {user_pk} month={month}: {e} (falling back to local points)"
            );
            None
        }
    };

    let token = match biyard.get_project_info().await {
        Ok(t) => Some(t),
        Err(e) => {
            crate::error!("Biyard get_project_info failed: {e}");
            None
        }
    };

    let biyard_points = biyard_balance.as_ref().map(|b| b.balance).unwrap_or(0);
    let points = std::cmp::max(local_points, biyard_points);
    let total_points = biyard_balance
        .as_ref()
        .map(|b| b.project_total_points)
        .filter(|v| *v > 0)
        .unwrap_or(points.max(1));
    let monthly_token_supply = biyard_balance
        .as_ref()
        .map(|b| b.monthly_token_supply)
        .unwrap_or(0);

    Ok(RewardsResponse {
        month,
        project_name: token.as_ref().map(|t| t.name.clone()).unwrap_or_default(),
        token_symbol: token
            .as_ref()
            .map(|t| t.symbol.clone())
            .unwrap_or_else(|| "RATEL".to_string()),
        total_points,
        points,
        monthly_token_supply,
        chain_id: token.as_ref().and_then(|t| t.chain_id),
        contract_address: token.as_ref().and_then(|t| t.contract_address.clone()),
    })
}

#[get("/api/users/points?username&month")]
pub async fn get_user_rewards_handler(
    username: String,
    month: Option<String>,
) -> Result<RewardsResponse> {
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

    fetch_rewards(user.pk.clone(), month).await
}
