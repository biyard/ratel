use crate::{features::rewards::RewardsResponse, types::MonthQuery, utils::time::current_month, *};
use aide::NoApi;
use axum::{Json, extract::{Query, State}};
use bdk::prelude::*;

pub async fn get_my_rewards_handler(
    State(AppState { biyard, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(MonthQuery { month }): Query<MonthQuery>,
) -> Result<Json<RewardsResponse>> {
    let month = month.unwrap_or_else(|| current_month());

    let balance = biyard
        .get_user_balance(user.pk.clone(), month.clone())
        .await?;

    let token = biyard.get_project_info().await?;

    Ok(Json(RewardsResponse {
        month,
        project_name: token.name,
        token_symbol: token.symbol,
        total_points: balance.project_total_points,
        points: balance.balance,
        monthly_token_supply: balance.monthly_token_supply,
    }))
}
