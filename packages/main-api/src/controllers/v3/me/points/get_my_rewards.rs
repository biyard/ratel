use crate::{utils::time::current_month, *};
use aide::NoApi;
use axum::{Json, extract::State};
use bdk::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct MyRewardsResponse {
    // Project Info
    pub project_name: String,
    pub token_symbol: String,

    pub month: String,
    pub total_points: i64,

    pub user_points: i64,
    pub monthly_token_supply: i64,
}

pub async fn get_my_rewards_handler(
    State(AppState { biyard, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
) -> Result<Json<MyRewardsResponse>> {
    let month = current_month();

    let balance = biyard
        .get_user_balance(user.pk.clone(), month.clone())
        .await?;

    let token = biyard.get_token().await?;

    // let exchange_ratio = if balance.project_total_points > 0 {
    //     balance.monthly_token_supply as f64 / balance.project_total_points as f64
    // } else {
    //     0.0
    // };

    // let estimated_tokens = balance.balance as f64 * exchange_ratio;

    Ok(Json(MyRewardsResponse {
        month,
        project_name: token.name,
        token_symbol: token.symbol,
        total_points: balance.project_total_points,
        user_points: balance.balance,
        monthly_token_supply: balance.monthly_token_supply,
    }))
}
