use super::error::ExchangePointsError;
use super::super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExchangeRequest {
    pub month: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExchangeResponse {
    pub month: String,
    pub exchanged_points: i64,
    pub minted_tokens: i64,
    pub token_balance: i64,
}

#[post("/api/me/points/exchange", user: crate::features::auth::User)]
pub async fn exchange_points_handler(body: ExchangeRequest) -> Result<ExchangeResponse> {
    let cfg = crate::common::CommonConfig::default();
    let biyard = cfg.biyard();

    let month = body
        .month
        .unwrap_or_else(|| utils::time::current_month());

    // Get current balance to determine exchange amount
    let balance = biyard
        .get_user_balance(user.pk.clone(), month.clone())
        .await?;

    if balance.balance <= 0 {
        return Err(ExchangePointsError::NoPointsAvailable.into());
    }

    let amount = balance.balance;

    // Calculate token amount based on share
    let estimated_tokens = if balance.project_total_points > 0 {
        ((amount as f64 / balance.project_total_points as f64)
            * balance.monthly_token_supply as f64)
            .round() as i64
    } else {
        0
    };

    if estimated_tokens <= 0 {
        return Err(ExchangePointsError::EstimatedTokensZero.into());
    }

    // Exchange points (deduct)
    biyard
        .exchange_points(user.pk.clone(), amount, month.clone())
        .await?;

    // Mint tokens
    let mint_result = biyard
        .mint_tokens(user.pk.clone(), estimated_tokens)
        .await?;

    Ok(ExchangeResponse {
        month,
        exchanged_points: amount,
        minted_tokens: estimated_tokens,
        token_balance: mint_result.balance,
    })
}
