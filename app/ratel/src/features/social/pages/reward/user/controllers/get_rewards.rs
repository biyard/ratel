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
    let cli = cfg.dynamodb();

    let month = month.unwrap_or_else(|| utils::time::current_month());

    // Scope-A: the balance is the local `User.points` (credited by reward
    // awards, debited by launchpad conversions). No console (Biyard) reads.
    let user =
        crate::features::auth::User::get(cli, user_pk.clone(), Some(crate::common::types::EntityType::User))
            .await?
            .unwrap_or_default();
    let points = user.points;

    Ok(RewardsResponse {
        month,
        project_name: String::new(),
        token_symbol: "RATEL".to_string(),
        total_points: points.max(1),
        points,
        monthly_token_supply: 0,
        chain_id: None,
        contract_address: None,
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
