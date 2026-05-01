use super::super::*;
use crate::common::services::PointTransactionResponse;

pub type ListTransactionsResponse = ListResponse<PointTransactionResponse>;

/// Enrich award rows with the `money_tree_level` / `money_tree_bonus`
/// fields so the frontend can render `RewardBreakdownChip`.
///
/// The upstream Biyard Points API does not persist Money Tree metadata
/// per-transaction. As a heuristic we use the user's *current* MoneyTree
/// level to estimate what the bonus on each award row would have been.
/// For users whose MoneyTree level hasn't changed between the time of
/// the award and now (the common case), this is exact. For users who
/// recently leveled up MoneyTree, older rows will be slightly
/// over-estimated — acceptable trade-off until upstream stores the
/// metadata. Non-award rows and level-0 users get `None` (chip hides).
#[cfg(feature = "server")]
async fn enrich_with_money_tree(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    rows: &mut [PointTransactionResponse],
) {
    use crate::features::character::leveling::multiplier_permille;
    use crate::features::character::models::CharacterSkill;
    use crate::features::character::types::SkillId;

    let level = match CharacterSkill::level_or_zero(cli, user_pk, SkillId::MoneyTree).await {
        Ok(l) if l > 0 => l,
        _ => return,
    };

    let permille = multiplier_permille(level);
    if permille <= 1000 {
        return;
    }

    for row in rows.iter_mut() {
        // Only enrich award rows with positive amounts. "spend" /
        // "exchange" / negative-amount rows never carry a Money Tree bonus.
        if !row.transaction_type.eq_ignore_ascii_case("award") {
            continue;
        }
        if row.amount <= 0 {
            continue;
        }

        // Inverse of `apply_permille`: base = round(amount · 1000 / permille).
        let base = (row.amount * 1000 + (permille as i64) / 2) / (permille as i64);
        let bonus = row.amount - base;
        if bonus > 0 {
            row.money_tree_level = Some(level);
            row.money_tree_bonus = Some(bonus);
        }
    }
}

#[cfg(feature = "server")]
async fn fetch_transactions(
    user_pk: Partition,
    month: Option<String>,
    bookmark: Option<String>,
) -> Result<ListTransactionsResponse> {
    let month = month.unwrap_or_else(|| utils::time::current_month());

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let biyard = cfg.biyard();
    let mut res = biyard
        .list_user_transactions(user_pk.clone(), month, bookmark, Some(10))
        .await?;

    enrich_with_money_tree(cli, &user_pk, &mut res.items).await;

    Ok(res)
}

#[get("/api/users/points/transactions?username&month&bookmark")]
pub async fn list_user_transactions_handler(
    username: String,
    month: Option<String>,
    bookmark: Option<String>,
) -> Result<ListTransactionsResponse> {
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

    fetch_transactions(user.pk.clone(), month, bookmark).await
}
