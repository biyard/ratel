use super::super::{dto::RewardsResponse, *};

#[get("/api/users/points?username&month")]
pub async fn get_user_rewards_handler(
    username: String,
    month: Option<String>,
) -> Result<RewardsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let biyard = cfg.biyard();

    let month = month.unwrap_or_else(|| utils::time::current_month());

    let (users, _) =
        crate::features::auth::User::find_by_username(cli, &username, Default::default()).await?;
    let user = users
        .into_iter()
        .find(|u| u.username == username)
        .ok_or(Error::NotFound(format!("User not found: {}", username)))?;

    let balance = biyard
        .get_user_balance(user.pk.clone(), month.clone())
        .await?;
    let token = biyard.get_project_info().await?;

    Ok(RewardsResponse {
        month,
        project_name: token.name,
        token_symbol: token.symbol,
        total_points: balance.project_total_points,
        points: balance.balance,
        monthly_token_supply: balance.monthly_token_supply,
    })
}
