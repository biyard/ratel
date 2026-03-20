use super::super::{dto::*, *};

#[get("/api/users/points/history?username&count")]
pub async fn get_monthly_history_handler(
    username: String,
    count: Option<i32>,
) -> Result<MonthlyHistoryResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let biyard = cfg.biyard();

    let (users, _) =
        crate::features::auth::User::find_by_username(cli, &username, Default::default()).await?;
    let user = users
        .into_iter()
        .find(|u| u.username == username)
        .ok_or(Error::NotFound(format!("User not found: {}", username)))?;

    let token = biyard.get_project_info().await?;
    let month_count = count.unwrap_or(6).min(12) as usize;

    let now = chrono::Utc::now();
    let mut months = Vec::new();
    for i in 0..month_count {
        let date = now - chrono::Duration::days(30 * i as i64);
        months.push(date.format("%Y-%m").to_string());
    }

    let mut items = Vec::new();
    let mut total_accumulated = 0i64;

    for month in &months {
        let balance = biyard
            .get_user_balance(user.pk.clone(), month.clone())
            .await
            .unwrap_or_default();

        total_accumulated += balance.balance;
        items.push(MonthlyPointSummary {
            month: month.clone(),
            points: balance.balance,
            total_points: balance.project_total_points,
            monthly_token_supply: balance.monthly_token_supply,
            is_swapped: false,
        });
    }

    Ok(MonthlyHistoryResponse {
        project_name: token.name,
        token_symbol: token.symbol,
        total_accumulated_points: total_accumulated,
        items,
    })
}
