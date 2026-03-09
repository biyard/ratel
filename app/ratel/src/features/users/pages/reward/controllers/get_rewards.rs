use super::super::{dto::RewardsResponse, *};

#[get("/api/me/points?month", user: crate::features::auth::User)]
pub async fn get_rewards_handler(month: Option<String>) -> Result<RewardsResponse> {
    let cfg = crate::common::CommonConfig::default();
    let biyard = cfg.biyard();

    let month = month.unwrap_or_else(|| utils::time::current_month());

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
