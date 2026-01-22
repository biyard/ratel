use crate::{controllers::v3::teams::dto::TeamPathParam, types::MonthQuery, utils::time::current_month, *};
use aide::NoApi;
use axum::{Json, extract::{Query, State}};
use bdk::prelude::*;

use crate::features::rewards::RewardsResponse;

pub async fn get_team_rewards_handler(
    State(AppState { biyard, .. }): State<AppState>,
    NoApi(_user): NoApi<User>,
    Path(TeamPathParam { team_pk }): Path<TeamPathParam>,
    Query(MonthQuery { month }): Query<MonthQuery>,
) -> Result<Json<RewardsResponse>> {
    let month = month.unwrap_or_else(|| current_month());

    let balance = biyard
        .get_user_balance(team_pk.clone(), month.clone())
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
