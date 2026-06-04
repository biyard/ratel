//! Inner callback handlers. Each takes already-verified inputs and
//! delegates point reads/writes to the Biyard console via `BiyardService`.

#![cfg(feature = "server")]

use crate::common::types::{EntityType, Partition};
use crate::common::utils::time::current_month;
use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
use crate::features::launchpad_partner::error::PartnerError;
use crate::features::launchpad_partner::models::LaunchpadDeduction;
use crate::features::launchpad_partner::types::{
    DeductBody, DeductResponse, HealthResponse, LookupResponse,
};

/// Read the user's current-month console balance.
pub async fn lookup(company_user_key: &str) -> Result<LookupResponse, PartnerError> {
    let cfg = LaunchpadPartnerConfig::default();
    let common = crate::common::CommonConfig::default();
    let biyard = common.biyard();
    let pk = Partition::User(company_user_key.to_string());

    let balance = biyard
        .get_user_balance(pk, current_month())
        .await
        .map_err(|e| {
            crate::error!("launchpad lookup: biyard balance failed: {e}");
            PartnerError::UnknownUser
        })?;

    Ok(LookupResponse {
        available_points: balance.balance,
        point_symbol: cfg.point_symbol.to_string(),
    })
}

/// Idempotently deduct points by issuing a console Exchange transaction.
pub async fn deduct(req: &DeductBody) -> Result<DeductResponse, PartnerError> {
    if req.point_amount <= 0 {
        return Err(PartnerError::InvalidAmount);
    }
    let common = crate::common::CommonConfig::default();
    let cli = common.dynamodb();
    let biyard = common.biyard();
    let pk = Partition::User(req.company_user_key.clone());

    // Idempotency: replay a stored result instead of double-spending.
    if let Ok(Some(existing)) = LaunchpadDeduction::get(
        cli,
        pk.clone(),
        Some(EntityType::LaunchpadDeduction(req.idempotency_key.clone())),
    )
    .await
    {
        return Ok(DeductResponse {
            brand_tx_id: existing.brand_tx_id,
            deducted_points: existing.point_amount,
            remaining_points: existing.remaining_points,
        });
    }

    let tx = biyard
        .exchange_points(pk.clone(), req.point_amount, current_month())
        .await
        .map_err(|e| {
            crate::error!("launchpad deduct: biyard exchange failed: {e}");
            PartnerError::Insufficient
        })?;

    // Exchange returns no remaining balance; re-query for it.
    let remaining = biyard
        .get_user_balance(pk, current_month())
        .await
        .map(|b| b.balance)
        .unwrap_or(0);

    let row = LaunchpadDeduction::new(
        &req.company_user_key,
        &req.idempotency_key,
        req.point_amount,
        &tx.transaction_id,
        remaining,
    );
    if let Err(e) = row.create(cli).await {
        crate::error!("launchpad deduct: idempotency row write failed: {e}");
    }

    Ok(DeductResponse {
        brand_tx_id: tx.transaction_id,
        deducted_points: req.point_amount,
        remaining_points: remaining,
    })
}

/// Health check — no console call; just confirms config + signature.
pub fn health() -> HealthResponse {
    let cfg = LaunchpadPartnerConfig::default();
    HealthResponse {
        ok: true,
        project_id: cfg.project_id.to_string(),
        service: "ratel".to_string(),
    }
}
