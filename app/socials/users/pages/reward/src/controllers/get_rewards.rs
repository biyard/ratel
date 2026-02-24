use crate::{dto::RewardsResponse, *};

#[get("/api/me/points?month", user: ratel_auth::User)]
pub async fn get_rewards_handler(month: Option<String>) -> Result<RewardsResponse> {
    use crate::models::BiyardClient;

    let month = month.unwrap_or_else(|| utils::time::current_month());

    let biyard = BiyardClient::new();
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
